use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Pos(usize, usize);

impl Pos {
    fn dist(&self, that: &Self) -> usize {
        self.0.max(that.0) - self.0.min(that.0) +
        self.1.max(that.1) - self.1.min(that.1)
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Game {
    hallway: [char; 11],
    rooms: [Vec<char>; 4],
}

impl Game {
    fn movable(&self) -> Vec<(char, Pos)> {
        let hall: Vec<(char, Pos)> = self.hallway.iter().enumerate()
            .flat_map(|(i, c)| if *c != '.' {Some((*c, Pos(i, 0)))} else {None})
            .filter(|(c, _)| {
                let room = target(*c)/2-1;
                self.rooms[room].iter().all(|x| x == c || *x == '.')
            })
            .collect();

        let rooms: Vec<(char, Pos)> = self.rooms.iter().enumerate()
            .flat_map(|(i, room)| {
                let idx = 2 + i * 2;

                room.iter()
                    .cloned()
                    .enumerate()
                    .filter(|(_, c)| *c != '.')
                    .filter(|(i, c)| (target(*c) != idx) || room[*i+1..].iter().any(|x| target(*x) != idx))
                    .map(|(i, c)| (c, Pos(idx, i+1)))
                    .next()
            })
            .collect();

        let mut result = Vec::with_capacity(hall.len() + rooms.len());
        result.extend_from_slice(&hall);
        result.extend_from_slice(&rooms);
        result
    }

    fn moves(&self, actor: char, pos: &Pos) -> Vec<Pos> {
        if pos.1 == 0 {
            // actor is in the hallway
            // it can go only to the own room (if the room contains no alien actors)
            // or it can go anywhere in the hallway (but not strictly above the rooms)

            let target = target(actor);
            let src = pos.0.min(target);
            let dst = pos.0.max(target);

            //println!("src={} dst={} target={}", src, dst, target);
            let has_path = (src..=dst)
                .filter(|i| *i != pos.0)
                .all(|i| self.hallway[i] == '.');

            let idx = target/2-1;
            let has_room = self.rooms[idx].iter().all(|x| *x == '.' || *x == actor);
            //println!("has_path={} has_room={}", has_path, has_room);

            if has_room && has_path {
                let pos = self.rooms[idx].iter().enumerate()
                    .rev()
                    .find(|(_, x)| **x == '.')
                    .map(|(i, _)| Pos(target, i+1))
                    .unwrap();
                return vec![pos];
            }
        }

        if pos.1 > 0 {
            let n = self.rooms[0].len();
            // actor is in the room
            // it can move to any available spot in the hall (but not strictly above the rooms)

            let target = target(actor);
            if pos.0 == target && pos.1 == n {
                return vec![];
            }

            let mut moves = Vec::new();

            let mut lo = pos.0 - 1;
            loop {
                if self.hallway[lo] == '.' {
                    if lo != 2 && lo != 4 && lo != 6 && lo != 8 {
                        moves.push(Pos(lo, 0));
                    }
                } else {
                    break;
                }
                if lo == 0 {
                    break;
                }
                lo -= 1;
            }

            let mut hi = pos.0 + 1;
            loop {
                if self.hallway[hi] == '.' {
                    if hi != 2 && hi != 4 && hi != 6 && hi != 8 {
                        moves.push(Pos(hi, 0));
                    }
                } else {
                    break;
                }
                hi += 1;
                if hi >= self.hallway.len() {
                    break;
                }
            }

            return moves;
        }

        vec![]
    }

    fn get(&mut self, pos: &Pos) -> &mut char {
        if pos.1 == 0 {
            self.hallway.get_mut(pos.0).unwrap()
        } else {
            let room = self.rooms.get_mut(pos.0/2-1).unwrap();
            room.get_mut(pos.1-1).unwrap()
        }
    }

    fn make_move(&mut self, actor: char, src: Pos, dst: Pos) -> usize {
        assert_eq!(*self.get(&src), actor);
        assert_eq!(*self.get(&dst), '.');

        *self.get(&dst) = actor;
        *self.get(&src) = '.';

        cost(actor) * dst.dist(&src)
    }

    fn done(&self) -> bool {
        self.rooms[0].iter().all(|x| *x == 'A') &&
        self.rooms[1].iter().all(|x| *x == 'B') &&
        self.rooms[2].iter().all(|x| *x == 'C') &&
        self.rooms[3].iter().all(|x| *x == 'D')
    }
}

fn target(actor: char) -> usize {
    match actor {
        'A' => 2,
        'B' => 4,
        'C' => 6,
        'D' => 8,
        _ => unreachable!()
    }
}

fn cost(actor: char) -> usize {
    match actor {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => unreachable!()
    }
}

#[derive(Eq, PartialEq, Clone)]
struct Entry(usize, Game);

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn bfs(game: Game) -> usize {
    let mut costs: HashMap<Game, usize> = HashMap::new();
    let mut queue = BinaryHeap::new();

    queue.push(Entry(0, game));

    while !queue.is_empty() {
        let Entry(cost, game) = queue.pop().unwrap();
        if game.done() {
            return cost;
        }

        if cost > costs.get(&game).cloned().unwrap_or(usize::MAX) {
            continue;
        }

        for (actor, at) in game.movable() {
            for to in game.moves(actor, &at) {
                let mut next = game.clone();
                let move_cost = next.make_move(actor, at, to);
                let new_cost = cost + move_cost;

                if new_cost < costs.get(&next).cloned().unwrap_or(usize::MAX) {
                    costs.insert(next.clone(), cost + move_cost);
                    queue.push(Entry(new_cost, next));
                }
            }
        }
    }

    0
}

fn main() {
    let game = Game {
        hallway: ['.'; 11],
        rooms: [
            vec!['C', 'D'],
            vec!['C', 'A'],
            vec!['B', 'B'],
            vec!['D', 'A'],
        ],
    };
    println!("{}", bfs(game));

    let game = Game {
        hallway: ['.'; 11],
        rooms: [
            vec!['C', 'D', 'D', 'D'],
            vec!['C', 'C', 'B', 'A'],
            vec!['B', 'B', 'A', 'B'],
            vec!['D', 'A', 'C', 'A'],
        ],
    };
    println!("{}", bfs(game));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moves_hallway() {
        // 0123456789X
        //      |
        // BA.C.C.B.AD
        //   . . . .
        //   D . X .

        let game = Game {
            hallway: ['B', 'A', '.', 'C', '.', 'C', '.', 'B', '.', 'A', 'D'],
            rooms: [
                vec!['.', 'D'],
                vec!['.', '.'],
                vec!['.', '.'],
                vec!['.', '.'],
            ],
        };

        assert_eq!(game.moves('C', &Pos(5, 0)), vec![Pos(6, 2)]);
    }

    #[test]
    fn test_moves_room() {
        // 0123456789X
        //   | |
        // AB.X.X.B..D
        //   C C . .
        //   D A . .

        let game = Game {
            hallway: ['B', 'A', '.', '.', '.', '.', '.', 'B', '.', '.', 'D'],
            rooms: [
                vec!['C', 'D'],
                vec!['C', 'A'],
                vec!['.', '.'],
                vec!['.', '.'],
            ],
        };

        assert_eq!(game.moves('C', &Pos(4, 1)), vec![Pos(3, 0), Pos(5, 0)]);
    }

    #[test]
    fn test_moves_final_1() {
        // ..........D
        //   . B C[D]
        //   A B C A

        let game = Game {
            hallway: ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'D'],
            rooms: [
                vec!['.', 'A'],
                vec!['B', 'B'],
                vec!['C', 'C'],
                vec!['D', 'A'],
            ],
        };

        let d = Pos(8, 1);

        assert_eq!(game.movable(), vec![('D', d)]);
        assert!(game.moves('D', &d).contains(&Pos(9, 0)));
    }

    #[test]
    fn test_moves_final_2() {
        // .........CC
        //   A . B D
        //   A . B D

        let game = Game {
            hallway: ['.', '.', '.', '.', '.', '.', '.', '.', '.', 'C', 'C'],
            rooms: [
                vec!['A', 'A'],
                vec!['.', '.'],
                vec!['B', 'B'],
                vec!['D', 'D'],
            ],
        };

        let p = Pos(6, 1);
        let b = ('B', p);
        assert_eq!(game.movable(), vec![b]);
        assert_eq!(game.moves('B', &p), vec![
            Pos(5, 0),
            Pos(3, 0),
            Pos(1, 0),
            Pos(0, 0),
            Pos(7, 0),
        ]);
    }

    #[test]
    fn test_moves_final_3() {
        // ..........A
        //   . B C D
        //   A B C D

        let game = Game {
            hallway: ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'A'],
            rooms: [
                vec!['.', 'A'],
                vec!['B', 'B'],
                vec!['C', 'C'],
                vec!['D', 'D'],
            ],
        };

        let p = Pos(10, 0);
        let a = ('A', p);
        assert_eq!(game.movable(), vec![a]);
        assert_eq!(game.moves('A', &p), vec![Pos(2, 1), ]);
    }

    #[test]
    fn test_done() {
        let game = Game {
            hallway: ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            rooms: [
                vec!['A', 'A'],
                vec!['B', 'B'],
                vec!['C', 'C'],
                vec!['D', 'D'],
            ],
        };

        assert!(game.done());
    }

    #[test]
    fn test_part1_answer() {
        let game = Game {
            hallway: ['.'; 11],
            rooms: [
                vec!['C', 'D'],
                vec!['C', 'A'],
                vec!['B', 'B'],
                vec!['D', 'A'],
            ],
        };

        assert_eq!(bfs(game), 15299);
    }

    #[test]
    fn test_part2_answer() {
        let game = Game {
            hallway: ['.'; 11],
            rooms: [
                vec!['C', 'D', 'D', 'D'],
                vec!['C', 'C', 'B', 'A'],
                vec!['B', 'B', 'A', 'B'],
                vec!['D', 'A', 'C', 'A'],
            ],
        };

        assert_eq!(bfs(game), 47193);
    }
}