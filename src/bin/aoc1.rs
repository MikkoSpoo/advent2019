// https://adventofcode.com/2019/day/1

// Fuel required to launch a given module is based on its mass.
// Specifically, to find the fuel required for a module, take its mass,
// divide by three, round down, and subtract 2
fn fuel_for_mass(m: i32) -> i32 {
	let r = m / 3;
	if r > 2 {
		r - 2
	} else {
		0
	}
}

fn fuel_for_mass_and_fuel(m: i32) -> i32 {
	let r = fuel_for_mass(m);
	if r == 0 {
		0
	} else {
		r + fuel_for_mass_and_fuel(r)
	}
}

fn fuels_for_input_components() -> (i32, i32) {
	// todo read from file
	let input: [i32; 100] = [
		126317,
	    64620,
	    139485,
	    77772,
	    104110,
	    103781,
	    62566,
	    76265,
	    122125,
	    54244,
	    113039,
	    142451,
	    118677,
	    54302,
	    143001,
	    81938,
	    110142,
	    115486,
	    128100,
	    81258,
	    126461,
	    81557,
	    147850,
	    138259,
	    73839,
	    96284,
	    149078,
	    59289,
	    125691,
	    102718,
	    142591,
	    110725,
	    56164,
	    76729,
	    133956,
	    140321,
	    57104,
	    125483,
	    115962,
	    52370,
	    74447,
	    121430,
	    96347,
	    116793,
	    76514,
	    60089,
	    113431,
	    66670,
	    120534,
	    117547,
	    113552,
	    131513,
	    118405,
	    85212,
	    57049,
	    118644,
	    54743,
	    95142,
	    58559,
	    85522,
	    73832,
	    141441,
	    97836,
	    98818,
	    104272,
	    100048,
	    99266,
	    97766,
	    115778,
	    51066,
	    132499,
	    129931,
	    119368,
	    91101,
	    139165,
	    106488,
	    105597,
	    66166,
	    117561,
	    94670,
	    123877,
	    63389,
	    70293,
	    79754,
	    105288,
	    128328,
	    130873,
	    54200,
	    120704,
	    57043,
	    71478,
	    133049,
	    102096,
	    82797,
	    62972,
	    121906,
	    77277,
	    97183,
	    112739,
	    135590];

	let mut total_component_fuel = 0;
	let mut total_component_and_fuel_fuel = 0;
	for ref_component_mass in input.iter() {
		let component_fuel = fuel_for_mass(*ref_component_mass);
		let component_and_fuel_fuel = fuel_for_mass_and_fuel(*ref_component_mass);
		total_component_fuel += component_fuel;
		total_component_and_fuel_fuel += component_and_fuel_fuel;
		// println!("Component mass {} needs {} fuel, with fuel for fuel {} total comp fuels {} total fuel {}",
		// 	     ref_component_mass, component_fuel, component_and_fuel_fuel,
		// 	     total_component_fuel, total_component_and_fuel_fuel);
	}
	(total_component_fuel, total_component_and_fuel_fuel)
}

fn main() {
	let (total_component_fuel, total_component_and_fuel_fuel) = fuels_for_input_components();
	println!("Phase1: {} Phase2: {}", total_component_fuel, total_component_and_fuel_fuel);
}

#[test]
fn t_fuel_for_mass() {
	// low values
    assert_eq!(fuel_for_mass(0), 0);
    assert_eq!(fuel_for_mass(1), 0);
    assert_eq!(fuel_for_mass(8), 0);
    assert_eq!(fuel_for_mass(9), 1);

    // examples
    assert_eq!(fuel_for_mass(12), 2);
    assert_eq!(fuel_for_mass(14), 2);
    assert_eq!(fuel_for_mass(1969), 654);
    assert_eq!(fuel_for_mass(100756), 33583);

    // assert_eq!(fuel_for_mass(4), 100); // wrong (test tests)
}

#[test]
fn t_correct_answer() {
	assert_eq!(fuels_for_input_components(), (3342351, 5010664));

}