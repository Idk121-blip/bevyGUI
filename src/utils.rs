use crate::components::{PLOT, PLOTUPDATE};

pub fn make_map(map_dim: usize) {
    let mut map = PLOT.lock().unwrap();
    let mut update_map = PLOTUPDATE.lock().unwrap();
    for x in 0..map_dim {
        map.push(vec![]);
        update_map.push(vec![]);
        for _y in 0..map_dim {
            map[x].push(None);
            update_map[x].push(None);
        }
    }
}
