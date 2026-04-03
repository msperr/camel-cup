use camel_cup::{Camel, State};
use std::collections::BTreeMap;

fn mk_map(entries: &[(i32, Vec<Camel>)]) -> BTreeMap<i32, Vec<Camel>> {
    let mut m = BTreeMap::new();
    for (k, v) in entries {
        m.insert(*k, v.clone());
    }
    m
}

#[test]
fn test_move_white_combinations() {
    let field = 1;
    let steps = 1;

    // Four possibilities for data[field]
    let v1 = vec![Camel::WHITE];
    let v2 = vec![Camel::WHITE, Camel::YELLOW];
    let v3 = vec![Camel::YELLOW, Camel::WHITE, Camel::ORANGE];
    let v4 = vec![Camel::YELLOW, Camel::WHITE];

    // Two possibilities for data[field + 1]
    let nv_empty: Vec<Camel> = vec![]; // represents absent key
    let nv_green = vec![Camel::GREEN];

    // Helper to run a single scenario: initial map entries, expected map entries
    let run = |initial_entries: &[(i32, Vec<Camel>)], expected_entries: &[(i32, Vec<Camel>)]| {
        let state = State::new(mk_map(initial_entries));
        let res = state.move_camel(Camel::WHITE, steps);
        let expected = mk_map(expected_entries);
        assert_eq!(res.data, expected);
    };

    // 1) v1, nv empty -> expected {2: [WHITE]}
    run(&[(field, v1.clone())], &[(field + 1, vec![Camel::WHITE])]);

    // 2) v1, nv = [GREEN] -> expected {2: [GREEN, WHITE]}
    run(
        &[(field, v1.clone()), (field + 1, nv_green.clone())],
        &[(field + 1, vec![Camel::GREEN, Camel::WHITE])],
    );

    // 3) v2 ([WHITE, YELLOW]), nv empty -> expected {2: [WHITE, YELLOW]}
    run(
        &[(field, v2.clone())],
        &[(field + 1, vec![Camel::WHITE, Camel::YELLOW])],
    );

    // 4) v2, nv = [GREEN] -> expected {2: [GREEN, WHITE, YELLOW]}
    run(
        &[(field, v2.clone()), (field + 1, nv_green.clone())],
        &[(field + 1, vec![Camel::GREEN, Camel::WHITE, Camel::YELLOW])],
    );

    // 5) v3 ([YELLOW, WHITE, ORANGE]), nv empty -> expected {1: [YELLOW], 2: [WHITE, ORANGE]}
    run(
        &[(field, v3.clone())],
        &[
            (field, vec![Camel::YELLOW]),
            (field + 1, vec![Camel::WHITE, Camel::ORANGE]),
        ],
    );

    // 6) v3, nv = [GREEN] -> expected {1: [YELLOW], 2: [GREEN, WHITE, ORANGE]}
    run(
        &[(field, v3.clone()), (field + 1, nv_green.clone())],
        &[
            (field, vec![Camel::YELLOW]),
            (field + 1, vec![Camel::GREEN, Camel::WHITE, Camel::ORANGE]),
        ],
    );

    // 7) v4 ([YELLOW, WHITE]), nv empty -> expected {1: [YELLOW], 2: [WHITE]}
    run(
        &[(field, v4.clone())],
        &[
            (field, vec![Camel::YELLOW]),
            (field + 1, vec![Camel::WHITE]),
        ],
    );

    // 8) v4, nv = [GREEN] -> expected {1: [YELLOW], 2: [GREEN, WHITE]}
    run(
        &[(field, v4.clone()), (field + 1, nv_green.clone())],
        &[
            (field, vec![Camel::YELLOW]),
            (field + 1, vec![Camel::GREEN, Camel::WHITE]),
        ],
    );
}

#[test]
fn test_move_all_camels_various_steps() {
    // initial map: 1:[WHITE], 3:[YELLOW, ORANGE, GREEN], 5:[BLUE]
    let initial = vec![
        (1, vec![Camel::WHITE]),
        (3, vec![Camel::YELLOW, Camel::ORANGE, Camel::GREEN]),
        (5, vec![Camel::BLUE]),
    ];
    let state = State::new(mk_map(&initial));

    // Helper to assert expected map
    let expect = |res_state: State, expected_entries: &[(i32, Vec<Camel>)]| {
        assert_eq!(res_state.data, mk_map(expected_entries));
    };

    // WHITE at 1:
    expect(
        state.move_camel(Camel::WHITE, 1),
        &[
            (2, vec![Camel::WHITE]),
            (3, vec![Camel::YELLOW, Camel::ORANGE, Camel::GREEN]),
            (5, vec![Camel::BLUE]),
        ],
    );
    expect(
        state.move_camel(Camel::WHITE, 2),
        &[
            (
                3,
                vec![Camel::YELLOW, Camel::ORANGE, Camel::GREEN, Camel::WHITE],
            ),
            (5, vec![Camel::BLUE]),
        ],
    );
    expect(
        state.move_camel(Camel::WHITE, 3),
        &[
            (3, vec![Camel::YELLOW, Camel::ORANGE, Camel::GREEN]),
            (4, vec![Camel::WHITE]),
            (5, vec![Camel::BLUE]),
        ],
    );

    // YELLOW at 3 (pos 0)
    expect(
        state.move_camel(Camel::YELLOW, 1),
        &[
            (1, vec![Camel::WHITE]),
            (4, vec![Camel::YELLOW, Camel::ORANGE, Camel::GREEN]),
            (5, vec![Camel::BLUE]),
        ],
    );
    expect(
        state.move_camel(Camel::YELLOW, 2),
        &[
            (1, vec![Camel::WHITE]),
            (
                5,
                vec![Camel::BLUE, Camel::YELLOW, Camel::ORANGE, Camel::GREEN],
            ),
        ],
    );
    expect(
        state.move_camel(Camel::YELLOW, 3),
        &[
            (1, vec![Camel::WHITE]),
            (5, vec![Camel::BLUE]),
            (6, vec![Camel::YELLOW, Camel::ORANGE, Camel::GREEN]),
        ],
    );

    // ORANGE at 3 (pos 1)
    expect(
        state.move_camel(Camel::ORANGE, 1),
        &[
            (1, vec![Camel::WHITE]),
            (3, vec![Camel::YELLOW]),
            (4, vec![Camel::ORANGE, Camel::GREEN]),
            (5, vec![Camel::BLUE]),
        ],
    );
    expect(
        state.move_camel(Camel::ORANGE, 2),
        &[
            (1, vec![Camel::WHITE]),
            (3, vec![Camel::YELLOW]),
            (5, vec![Camel::BLUE, Camel::ORANGE, Camel::GREEN]),
        ],
    );
    expect(
        state.move_camel(Camel::ORANGE, 3),
        &[
            (1, vec![Camel::WHITE]),
            (3, vec![Camel::YELLOW]),
            (5, vec![Camel::BLUE]),
            (6, vec![Camel::ORANGE, Camel::GREEN]),
        ],
    );

    // GREEN at 3 (pos 2)
    expect(
        state.move_camel(Camel::GREEN, 1),
        &[
            (1, vec![Camel::WHITE]),
            (3, vec![Camel::YELLOW, Camel::ORANGE]),
            (4, vec![Camel::GREEN]),
            (5, vec![Camel::BLUE]),
        ],
    );
    expect(
        state.move_camel(Camel::GREEN, 2),
        &[
            (1, vec![Camel::WHITE]),
            (3, vec![Camel::YELLOW, Camel::ORANGE]),
            (5, vec![Camel::BLUE, Camel::GREEN]),
        ],
    );
    expect(
        state.move_camel(Camel::GREEN, 3),
        &[
            (1, vec![Camel::WHITE]),
            (3, vec![Camel::YELLOW, Camel::ORANGE]),
            (5, vec![Camel::BLUE]),
            (6, vec![Camel::GREEN]),
        ],
    );

    // BLUE at 5 (pos 0)
    expect(
        state.move_camel(Camel::BLUE, 1),
        &[
            (1, vec![Camel::WHITE]),
            (3, vec![Camel::YELLOW, Camel::ORANGE, Camel::GREEN]),
            (6, vec![Camel::BLUE]),
        ],
    );
    expect(
        state.move_camel(Camel::BLUE, 2),
        &[
            (1, vec![Camel::WHITE]),
            (3, vec![Camel::YELLOW, Camel::ORANGE, Camel::GREEN]),
            (7, vec![Camel::BLUE]),
        ],
    );
    expect(
        state.move_camel(Camel::BLUE, 3),
        &[
            (1, vec![Camel::WHITE]),
            (3, vec![Camel::YELLOW, Camel::ORANGE, Camel::GREEN]),
            (8, vec![Camel::BLUE]),
        ],
    );
}
