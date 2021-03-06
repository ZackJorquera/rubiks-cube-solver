// TODO: make lib
use statrs::statistics::Statistics;

mod rubiks;
mod solver;
mod rubiks_render;

use solver::RubiksCubeSolver;

use std::time::Instant;

use std::io;

fn time_solves()
{
    // time heuristics table
    let ths = Instant::now();
    for _ in 0..9
    {
        solver::HeuristicsTables::new().calc_corner_heuristics_table();
    }
    let mut htable = solver::HeuristicsTables::new();
    htable.calc_corner_heuristics_table();
    let htime = ths.elapsed().as_secs_f64() / 10.0;
    println!("time to calc corner heuristics table: {}", htime);

    let mut rsolver = solver::RubiksCubeSolver::new();
    rsolver.add_heuristics_table(htable);

    let mut idastar_times: Vec<Vec<f64>> = vec![];
    let mut dpll_times: Vec<Vec<f64>> = vec![];
    let mut dpllp2_times: Vec<Vec<f64>> = vec![];

    println!("m,k,2,3,4,5,6,7,8,9,10");

    for k in 0..15
    {
        let mut idastar_times_for_k: Vec<f64> = vec![];
        let mut dpll_times_for_k: Vec<f64> = vec![];
        let mut dpllp2_times_for_k: Vec<f64> = vec![];
        for n in 2..=7
        {
            let mut idastar_times_for_runs: Vec<f64> = vec![];
            let mut dpll_times_for_runs: Vec<f64> = vec![];
            let mut dpllp2_times_for_runs: Vec<f64> = vec![];
            for _ in 0..10
            {
                let (state, _) = rubiks::RubiksCubeState::rnd_scramble(n, k);

                let tidass = Instant::now();
                let _ = rsolver.solve_with_idastar(&state);
                idastar_times_for_runs.push(tidass.elapsed().as_secs_f64());

                let dplls = Instant::now();
                let _ = rsolver.solve_dpll(&state, k);
                dpll_times_for_runs.push(dplls.elapsed().as_secs_f64());

                if n <= 5
                {
                    let dpllp2s = Instant::now();
                    let _ = rsolver.solve_dpll(&state, k+2);
                    dpllp2_times_for_runs.push(dpllp2s.elapsed().as_secs_f64());
                }
            }
            idastar_times_for_k.push(idastar_times_for_runs.mean());
            dpll_times_for_k.push(dpll_times_for_runs.mean());
            dpllp2_times_for_k.push(dpllp2_times_for_runs.mean());
        }
        println!("ida*,{}{}", k, idastar_times_for_k.iter().fold(String::from(""), |s, e| format!("{},{}", s, e)));
        idastar_times.push(idastar_times_for_k);

        println!("dpll,{}{}", k, dpll_times_for_k.iter().fold(String::from(""), |s, e| format!("{},{}", s, e)));
        dpll_times.push(dpll_times_for_k);

        println!("dpllp2,{}{}", k, dpllp2_times_for_k.iter().fold(String::from(""), |s, e| format!("{},{}", s, e)));
        dpllp2_times.push(dpllp2_times_for_k);
    }
}

fn solve_given(show_cubes: bool)
{
    // wwoowwbgrgbybggygroogrrrgrrygybbywwogoooowbybwybyyrrbw
    // gowgwyywowgowowgorrryygogwowworwgggywgggyooyorwgggbboborwrwrrwrwogggworwrwgybrrrgyyrybbbbbbbbbbooooobwbgrgoybyryoboryobobyyyyybybwyryrwyryrwrgggwbbbrw
    // let mut solver = RubiksCubeSolver::from_state_string(&String::from("yworrygogbwrwbyoobyrggwb"));
    // let t0 = Instant::now();
    // solver.calc_heuristics_table();
    // println!("Done calculating heuristics table in {} secs.", t0.elapsed().as_secs_f64());
    // //let t0 = Instant::now();
    // let res0 = solver.solver_2x2x2_heuristics_table(14);
    // println!("Found {:?} turn solution: {}", res0.clone().1.map(|l| l.turns.len()), res0.1.unwrap());

    //let mut solver = RubiksCubeSolver::from_state(rubiks::RubiksCubeState::std_solved_nxnxn(2));
    let mut solver = RubiksCubeSolver::new();
    let t0 = Instant::now();
    solver.calc_new_heuristics_table();
    println!("Done calculating heuristics table in {} secs.", t0.elapsed().as_secs_f64());

    loop
    {
        println!("Input cube state:");

        let mut input = String::new();
        let input_state;
        match io::stdin().read_line(&mut input)
        {
            Ok(_) => 
            {
                match rubiks::RubiksCubeState::from_state_string(&input.trim().to_owned()) 
                {
                    Ok(new_state) => {
                        println!("We got:\n{:?}", &new_state);
                        if show_cubes { rubiks_render::RubikDrawer::from_state(new_state.clone()).show(); }
                        input_state = new_state;
                    },
                    Err(e) => {
                        println!("Failed to read state, error: {}", e);
                        continue;
                    }
                }

                if input_state.size() == 2
                {
                    match solver.solver_2x2x2_with_heuristics_table(&input_state)
                    {
                        Ok(the_move) => println!("Solution: {}", the_move),
                        Err(err) => println!("No Solution: {:?}", err),
                    }
                }
                else
                {
                    match solver.solve_with_idastar(&input_state)
                    //match solver.solve_dpll(&input_state, 10)
                    {
                        Ok(the_move) => println!("Solution: {}", the_move),
                        Err(err) => println!("No Solution: {:?}", err),
                    }
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}

fn quick_and_dirty_rend()
{
    let mut state = rubiks::RubiksCubeState::std_solved_nxnxn(5);
    rubiks_render::RubikDrawer::from_state(state.clone()).show();

    let the_move = rubiks::Move{turns: vec![rubiks::Turn::FaceBased{face: rubiks::Face::Up, inv: true, num_in:0, cube_size: 3},
                                            rubiks::Turn::FaceBased{face: rubiks::Face::Front, inv: true,  num_in:0, cube_size: 3},
                                            rubiks::Turn::FaceBased{face: rubiks::Face::Left, inv: true, num_in:0, cube_size: 3}]};

    state.do_move(&the_move);

    rubiks_render::RubikDrawer::from_state(state).show();
}

fn test_draw()
{
    let n = 5;
    let m = 3;
    let s = 6*n+2*m;

    let ls: Vec<[u8; 3]> = vec![[0, 1, 1], [1, 1, 0], [1, 1, 1], [1, 0, 0], [0, 0, 0]];

    // let n = 3;
    // let m = 2;
    // let s = 6*n+2*m;

    // let ls: Vec<[u8; 2]> = vec![[1, 1], [0, 1], [0, 0]];

    let bs = ls.clone().into_iter().enumerate().map(|(i,l)| 
    {
        let mut a_i = rubiks::Move::empty();
        for (j, bit) in l.iter().enumerate()
        {
            if *bit != 0 
            { 
                a_i *= rubiks::Turn::AxisBased{
                    axis: rubiks::Axis::X, pos_rot: true, index: (j+1) as isize, cube_size: s}.as_move();
            }
        }
        let z_m_i = rubiks::Turn::AxisBased{
                    axis: rubiks::Axis::Z, pos_rot: true, index: (m+i+1) as isize, cube_size: s}.as_move();

        a_i.clone() * z_m_i * a_i.invert()
    });

    let mut state = rubiks::RubiksCubeState::std_solved_nxnxn(s);

    let mut a_1 = rubiks::Move::empty();
    for (j, bit) in ls[0].iter().enumerate()
    {
        if *bit != 0 
        { 
            a_1 *= rubiks::Turn::AxisBased{
                axis: rubiks::Axis::X, pos_rot: true, index: (j+1) as isize, cube_size: s}.as_move();
        }
    }

    let mut tb = rubiks::Move::empty();
    let t;

    for bi in bs.clone()//.rev() // rev doesn't matter, all bis commute
    {
        //println!("{}", bi);
        tb *= bi;
    }

    t = tb * a_1;
    
    println!("{}\n{:?}", t,state);
    rubiks_render::RubikDrawer::from_state(state.clone()).show();
    for turn in t
    {
        state.turn(turn);
        rubiks_render::RubikDrawer::from_state(state.clone()).show();
    }
    // state.do_move(&t.clone());

    rubiks_render::RubikDrawer::from_state(state.clone()).show();

    let soln = rubiks::Move{turns: vec![rubiks::Turn::AxisBased{axis: rubiks::Axis::Z, pos_rot: false, index:4, cube_size: s},
                                        rubiks::Turn::AxisBased{axis: rubiks::Axis::X, pos_rot: true,  index:1, cube_size: s},
                                        rubiks::Turn::AxisBased{axis: rubiks::Axis::Z, pos_rot: false, index:6, cube_size: s},
                                        rubiks::Turn::AxisBased{axis: rubiks::Axis::X, pos_rot: false, index:3, cube_size: s},
                                        rubiks::Turn::AxisBased{axis: rubiks::Axis::Z, pos_rot: false, index:5, cube_size: s},
                                        rubiks::Turn::AxisBased{axis: rubiks::Axis::X, pos_rot: false, index:2, cube_size: s},
                                        rubiks::Turn::AxisBased{axis: rubiks::Axis::Z, pos_rot: false, index:7, cube_size: s},
                                        rubiks::Turn::AxisBased{axis: rubiks::Axis::X, pos_rot: false, index:1, cube_size: s},
                                        rubiks::Turn::AxisBased{axis: rubiks::Axis::Z, pos_rot: false, index:8, cube_size: s}]};
    

    rubiks_render::RubikDrawer::from_state(state.clone()).show();
    for turn in soln.clone()
    {
        state.turn(turn);
        rubiks_render::RubikDrawer::from_state(state.clone()).show();
    }
    // state.do_move(&soln);

    println!("{}\n{:?}\nsolved: {}", soln, state, state.is_solved());
}

fn main() 
{
    time_solves();

    let show_cubes = std::env::args().nth(1).map(|s| s.to_lowercase().contains("show")) == Some(true);

    if show_cubes
    {
        quick_and_dirty_rend();
        test_draw();
    }

    solve_given(show_cubes);
    // let (r_state, _turns) = rubiks::RubiksCubeState::rnd_scramble(2, 100);
    // //println!("{}\n{:?}", turns, r_state);
    // let mut solver = RubiksCubeSolver::from_state(r_state);
    // solver.calc_heuristics_table();
    // let t0 = Instant::now();
    // let res0 = solver.solver_dpll_2x2x2(14);
    // println!("Found {:?} turn solution in {} secs.", res0.1.map(|l| l.turns.len()), t0.elapsed().as_secs_f64());

    // let solved_3x3_state = "WWWWWWWWWGGGGGGGGGRRRRRRRRRBBBBBBBBBOOOOOOOOOYYYYYYYYY".to_owned();
    // let state = rubiks::RubiksCubeState::from_state_string(&solved_3x3_state);
    // println!("{:?}", state);
    
    // let solved_3x3_state_str = "WWWWWWWWWGGGGGGGGGRRRRRRRRRBBBBBBBBBOOOOOOOOOYYYYYYYYY".to_owned();
    // let mut r_state = rubiks::RubiksCubeState::from_state_string(&solved_3x3_state_str);
    // r_state.turn(rubiks::Face::Left, true, 0);
    // r_state.turn(rubiks::Face::Up, false, 0);
    // r_state.turn(rubiks::Face::Down, false, 0);

    // let (r_state, turns) = rubiks::RubiksCubeState::rnd_scramble(3, 100);
    // println!("{}\n{:?}", turns, r_state);
    let mut solver = RubiksCubeSolver::new();
    let t0 = Instant::now();
    solver.calc_new_heuristics_table();
    println!("Done calculating heuristics table in {} secs.", t0.elapsed().as_secs_f64());

    // t0 = Instant::now();
    // let res1 = solver.solve_dpll(15);
    // println!("Found solution in {} secs.\n{:?}", t0.elapsed().as_secs_f64(), res1);
    // t0 = Instant::now();
    // let res12 = solver.new_solve_dpll(15);
    // println!("Found solution in {} secs.\n{:?}", t0.elapsed().as_secs_f64(), res12);
    // if let (_, Some(r)) = res1
    // {
    //     println!("{}", r);
    // }

    // t0 = Instant::now();
    // let res2 = solver.solve_dpll(20);
    // println!("Found solution in {} secs.\n{:?}", t0.elapsed().as_secs_f64(), res2);
    // t0 = Instant::now();
    // let res22 = solver.new_solve_dpll(20);
    // println!("Found solution in {} secs.\n{:?}", t0.elapsed().as_secs_f64(), res22);
    // if let (_, Some(r)) = res2
    // {
    //     println!("{}", r);
    // }
}