use crate::life::GameOfLife;

fn place(life: &mut GameOfLife, ox: i32, oy: i32, cells: &[(i32, i32)]) {
    for &(dx, dy) in cells {
        life.set_alive(ox + dx, oy + dy);
    }
}

pub fn glider(life: &mut GameOfLife, ox: i32, oy: i32) {
    place(life, ox, oy, &[(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)]);
}

pub fn lwss(life: &mut GameOfLife, ox: i32, oy: i32) {
    place(
        life,
        ox,
        oy,
        &[(1, 0), (4, 0), (0, 1), (0, 2), (4, 2), (0, 3), (1, 3), (2, 3), (3, 3)],
    );
}

pub fn gosper_glider_gun(life: &mut GameOfLife, ox: i32, oy: i32) {
    place(
        life,
        ox,
        oy,
        &[
            (24, 0),
            (22, 1), (24, 1),
            (12, 2), (13, 2), (20, 2), (21, 2), (34, 2), (35, 2),
            (11, 3), (15, 3), (20, 3), (21, 3), (34, 3), (35, 3),
            (0, 4), (1, 4), (10, 4), (16, 4), (20, 4), (21, 4),
            (0, 5), (1, 5), (10, 5), (14, 5), (16, 5), (17, 5), (22, 5), (24, 5),
            (10, 6), (16, 6), (24, 6),
            (11, 7), (15, 7),
            (12, 8), (13, 8),
        ],
    );
}

pub fn pulsar(life: &mut GameOfLife, ox: i32, oy: i32) {
    let arm = [2, 3, 4, 8, 9, 10];
    for &d in &arm {
        life.set_alive(ox + d, oy);
        life.set_alive(ox + d, oy + 5);
        life.set_alive(ox + d, oy + 7);
        life.set_alive(ox + d, oy + 12);
        life.set_alive(ox, oy + d);
        life.set_alive(ox + 5, oy + d);
        life.set_alive(ox + 7, oy + d);
        life.set_alive(ox + 12, oy + d);
    }
}

pub fn pentadecathlon(life: &mut GameOfLife, ox: i32, oy: i32) {
    place(
        life,
        ox,
        oy,
        &[
            (1, 0), (1, 1), (0, 2), (2, 2), (1, 3), (1, 4),
            (1, 5), (1, 6), (0, 7), (2, 7), (1, 8), (1, 9),
        ],
    );
}

pub fn beacon(life: &mut GameOfLife, ox: i32, oy: i32) {
    place(life, ox, oy, &[(0, 0), (1, 0), (0, 1), (3, 2), (2, 3), (3, 3)]);
}

pub fn toad(life: &mut GameOfLife, ox: i32, oy: i32) {
    place(life, ox, oy, &[(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)]);
}

pub fn r_pentomino(life: &mut GameOfLife, ox: i32, oy: i32) {
    place(life, ox, oy, &[(1, 0), (2, 0), (0, 1), (1, 1), (1, 2)]);
}

pub fn diehard(life: &mut GameOfLife, ox: i32, oy: i32) {
    place(
        life,
        ox,
        oy,
        &[(6, 0), (0, 1), (1, 1), (1, 2), (5, 2), (6, 2), (7, 2)],
    );
}

pub fn acorn(life: &mut GameOfLife, ox: i32, oy: i32) {
    place(
        life,
        ox,
        oy,
        &[(1, 0), (3, 1), (0, 2), (1, 2), (4, 2), (5, 2), (6, 2)],
    );
}

pub fn block(life: &mut GameOfLife, ox: i32, oy: i32) {
    place(life, ox, oy, &[(0, 0), (1, 0), (0, 1), (1, 1)]);
}