// Slot	            M	T	W	Th	F
// 0 8:30 – 9:50	A1	B1	A2	C2	B2
// 1 10:00 - 11:20	C1	D1	E1	D2	E2
// 2 11:30 - 12:50	F1	G1	H2	F2	G2
// 3 13:00 - 14:00	T1	T2	T3	O1	O2
// 4 14:00 - 15:20	I1	J1	I2	K2	J2
// 5 15:30 - 16:50	K1	L1	M1	L2	M2
// 6 17:00 - 18:20	H1	N1	P1	N2	P2

// create a hashmap to store the timeslots where A1, B1, etc. are the keys and the values are the timeslots and the days

use std::collections::HashMap;

pub fn maap() -> HashMap<String, (u8, String)> {
    let mut map = HashMap::new();

    map.insert("A1".to_string(), (0, "MO".to_string()));
    map.insert("B1".to_string(), (0, "TU".to_string()));
    map.insert("A2".to_string(), (0, "WE".to_string()));
    map.insert("C2".to_string(), (0, "TH".to_string()));
    map.insert("B2".to_string(), (0, "FR".to_string()));

    map.insert("C1".to_string(), (1, "MO".to_string()));
    map.insert("D1".to_string(), (1, "TU".to_string()));
    map.insert("E1".to_string(), (1, "WE".to_string()));
    map.insert("D2".to_string(), (1, "TH".to_string()));
    map.insert("E2".to_string(), (1, "FR".to_string()));

    map.insert("F1".to_string(), (2, "MO".to_string()));
    map.insert("G1".to_string(), (2, "TU".to_string()));
    map.insert("H2".to_string(), (2, "WE".to_string()));
    map.insert("F2".to_string(), (2, "TH".to_string()));
    map.insert("G2".to_string(), (2, "FR".to_string()));

    map.insert("T1".to_string(), (3, "MO".to_string()));
    map.insert("T2".to_string(), (3, "TU".to_string()));
    map.insert("T3".to_string(), (3, "WE".to_string()));
    map.insert("O1".to_string(), (3, "TH".to_string()));
    map.insert("O2".to_string(), (3, "FR".to_string()));

    map.insert("I1".to_string(), (4, "MO".to_string()));
    map.insert("J1".to_string(), (4, "TU".to_string()));
    map.insert("I2".to_string(), (4, "WE".to_string()));
    map.insert("K2".to_string(), (4, "TH".to_string()));
    map.insert("J2".to_string(), (4, "FR".to_string()));

    map.insert("K1".to_string(), (5, "MO".to_string()));
    map.insert("L1".to_string(), (5, "TU".to_string()));
    map.insert("M1".to_string(), (5, "WE".to_string()));
    map.insert("L2".to_string(), (5, "TH".to_string()));
    map.insert("M2".to_string(), (5, "FR".to_string()));

    map.insert("H1".to_string(), (6, "MO".to_string()));
    map.insert("N1".to_string(), (6, "TU".to_string()));
    map.insert("P1".to_string(), (6, "WE".to_string()));
    map.insert("N2".to_string(), (6, "TH".to_string()));
    map.insert("P2".to_string(), (6, "FR".to_string()));

    map
}
