pub mod group;
mod persons;

use core::time;
use std::env;
use std::fs::{self, remove_file};

use rand::Rng;
use std::process::exit;
use std::time::Instant;
use std::{collections::HashSet, error::Error, io, slice::SliceIndex};
use uuid::Uuid;
use xlsxwriter::{
    DateTime as XLSDateTime, Format, FormatAlignment, FormatColor, FormatUnderline, Workbook,
    Worksheet,
};

use crate::{group::Group, persons::Persons};

//SECTION - Start with: cargo run -- valueForDoubles valueForTrible value for Quadruple example-filename.txt
//SECTION - Otherwise you can start with : cargo run or: cargo run example-filename.txt

fn main() {
    const PERSONS_MIN: i32 = 120;
    const PERSONS_MAX: i32 = 300;
    const pers: i32 = 80;
    const PERSONS_Test_2: i32 = 200;
    let group_size1: i32 = 5;
    let group_size3 = 5;
    let group_size2 = 6;
    let mut filename = String::new();
    let mut new_input = String::new();
    let mut default_value_double = 0.1;
    let mut default_value_tribble = 0.2;
    let mut default_value_quadruple = 0.3;
    println!("Bitte geben Sie einen Filename für ihr Excel Doc ein, oder d wenn Sie den default (Excel_new.xlsx) nutzen wollen");
    io::stdin().read_line(&mut filename).expect("Error");
    println!("{}", filename);
    /*if filename.starts_with("d") && filename.ends_with('\n') {
        print!("Lol");
    }*/
    if (filename == "d\n".to_string()) {
        filename = "Excel_new.xlsx".to_string();
    }
    println!("Bitte geben Sie den Wert für die Güteberechnung bei Dopplungen ein, oder d für Default(0.1)");
    io::stdin().read_line(&mut new_input).expect("Error");
    if new_input != "d\n".to_string() {
        default_value_double = new_input.trim().parse().unwrap();
    }
    println!("Bitte geben Sie den Wert für die Güteberechnung bei Tribletten ein, oder d für Default(0.2)");
    new_input = String::new();
    io::stdin().read_line(&mut new_input).expect("Error");
    if new_input != "d\n".to_string() {
        default_value_tribble = new_input.trim().parse().unwrap();
    }
    println!("Bitte geben Sie den Wert für die Güteberechnung bei Quadruble ein, oder d für Default(0.3)");
    new_input = String::new();

    io::stdin().read_line(&mut new_input).expect("Error");
    if new_input != "d\n".to_string() {
        default_value_quadruple = new_input.trim().parse().unwrap();
    }
    print!("{}", default_value_double);
    println!("{}", default_value_tribble);
    println!("{}", default_value_quadruple);
    println!("{}", filename);

    /*let perm = Persons {
        name: 0,
        number_of_meetings: [0; 300],
        peoplemet: [0; 300],
    };*/

    //let list_of_persons_min: [Persons; 300] = [perm; 300];
    //let t = PERSONS_MIN / group_size1;
    //const minP: usize = 70 / 4;
    //let list_of_persons_min1: [Persons; PERSONS_MIN as usize] = [perm; 70];
    //const maxP: usize = 300 / 4;
    //let list_of_persons_max1: [Persons; personsMax as usize / 6] = [perm; 50];
    //let list_of_persons_max: LinkedList<Persons> = LinkedList::new();
    //let group_size2: i32 = 5;
    //let group_size3: i32 = 6;
    const SHIFTS: i32 = 10;
    //let shiftInU = shifts as usize;

    //let mut result: [Group; shifts as usize];
    //list_of_persons_min = create_persons(personsMin);

    //Bis zu 300 Leute, aufgrund von Speicheroverflows aufteilung in 50er gruppen
    let person_met = Vec::new();
    let p0 = Persons {
        //edges: todo!(),
        peoplemet: person_met,
        name: 0,
        number_of_meetings: Vec::new(),
    };
    let p_in_group = vec![p0; 4];
    let current_group: Group = Group {
        group_size: group_size1,
        persons_in_group: p_in_group,
    };
    //300 /4 = 75 + 5(puffer);
    //let mut global_group_list = [current_group; 80];
    let all_pers = create_persons(pers);
    let mut existing_pers = Vec::new();
    let help_pers = all_pers.clone();
    for p in help_pers {
        existing_pers.push(p.name);
    }

    let t = all_pers.clone();

    //println!("{}", all_pers.len());
    let shifts = calculate(all_pers, group_size1, SHIFTS, pers, existing_pers);
    let mut t = shifts.clone();
    let mut quality = calculateQuality(
        //quality an stelle 4 ist gesammtquali
        default_value_double,
        default_value_tribble,
        default_value_quadruple,
        shifts,
    );
    let start = Instant::now();
    let mut still_searching = true;
    let m = t.clone();
    let testqual = calculateQuality(
        default_value_double,
        default_value_tribble,
        default_value_quadruple,
        m.clone(),
    );
    build_file(m, "default.xlsx".to_string(), testqual);
    let name_10Min = "10Minute_".to_string() + &filename.clone();
    let name_20Min = "20Minute_".to_string() + &filename.clone();
    let name_5Min = "5Minute_".to_string() + &filename.clone();
    let name_1Std = "1Stunde_".to_string() + &filename.clone();
    while still_searching {
        let new_result = try_better_result4(t.clone(), group_size1, SHIFTS);
        //let new_result = try_better_result3(t.clone(), group_size2, SHIFTS);

        //let new_result = try_better_result2(t.clone(), group_size2, SHIFTS);
        //let new_result = try_better_result8(t.clone(), group_size2, SHIFTS);
        //let new_result = try_better_result2(new_result1, group_size2, SHIFTS);
        let help = new_result.clone();
        let newqual = calculateQuality(
            default_value_double,
            default_value_tribble,
            default_value_quadruple,
            new_result,
        );
        if newqual[4] < quality[4] {
            quality = newqual;
            t = help;
        }

        let duration = start.elapsed();
        println!("{}", duration.as_secs());
        if duration.as_secs() == 600 {
            //10 Minuten
            build_file(t.clone(), name_10Min.to_string(), quality);
        }
        if duration.as_secs() == 300 {
            // 5Minuten

            build_file(t.clone(), name_5Min.to_string(), quality);
        }
        if duration.as_secs() == 1200 {
            // 20Minuten

            build_file(t.clone(), name_20Min.to_string(), quality);
        }
        if duration.as_secs() == 3600 {
            // 60Minuten

            build_file(t.clone(), name_1Std.to_string(), quality);
        }
        if duration.as_secs() == 43200 {
            // 12 Stunden
            still_searching = false;
        }
    }
    build_file(t, filename, quality);
    //println!("{:?}", t.clone());
    println!("finished");
}

fn create_persons(count_persons: i32) -> Vec<Persons> {
    let mut person_list = Vec::new();
    for y in 0..count_persons {
        let p = Persons {
            peoplemet: Vec::new(),
            name: y,
            number_of_meetings: Vec::new(),
        };
        person_list.push(p);
    }
    println!("created");
    person_list
}

fn calculate(
    list_of_persons: Vec<Persons>,
    group_size: i32,
    time_shifts: i32,
    persons_max: i32,
    existing_pers: Vec<i32>,
) -> Vec<Vec<Group>> {
    let p0 = Persons {
        //edges: todo!(),
        peoplemet: Vec::new(),
        name: 0,
        number_of_meetings: Vec::new(),
    };
    let p_in_group = vec![p0; 4];
    let current_group: Group = Group {
        group_size,
        persons_in_group: p_in_group,
    };
    //timeslots ==10 ; 80 Gruppen, weil maximal 300 Personen /4 = 75 plus Puffer
    let new_vec: Vec<Group> = Vec::new();

    //let mut all_shifts = [Vec::new(); 10];
    let mut all_shifts = vec![Vec::new(); time_shifts as usize];

    //let mut list_of_persons_max12 = [p0; 52];
    let mut list_of_persons_max1 = Vec::new();
    let mut ex_pers1 = Vec::new();
    //let mut list_of_persons_max21 = [p0; 52];
    let mut list_of_persons_max2 = Vec::new();
    let mut ex_pers2 = Vec::new();
    //let mut list_of_persons_max31 = [p0; 52];
    let mut list_of_persons_max3 = Vec::new();
    let mut ex_pers3 = Vec::new();
    //let mut list_of_persons_max41 = [p0; 52];
    let mut list_of_persons_max4 = Vec::new();
    let mut ex_pers4 = Vec::new();
    //let mut list_of_persons_max51 = [p0; 52];
    let mut list_of_persons_max5 = Vec::new();
    let mut ex_pers5 = Vec::new();
    //let mut list_of_persons_max61 = [p0; 52];
    let mut list_of_persons_max6 = Vec::new();
    let mut ex_pers6 = Vec::new();
    for number in 0..persons_max {
        if number < 60 {
            //list_of_persons_max12[number as usize] = list_of_persons[number as usize];

            let var = list_of_persons.get(number as usize);
            if var != None {
                let p = var.unwrap().clone();
                list_of_persons_max1.push(p);
                ex_pers1.push(existing_pers.get(number as usize).unwrap().clone());
            }
        } else if number < 120 {
            let new_number = number - 60;
            let var = list_of_persons.get(number as usize);
            if var != None {
                let p = var.unwrap().clone();
                list_of_persons_max2.push(p);
                ex_pers2.push(existing_pers.get(number as usize).unwrap().clone());
            }
        } else if number < 180 {
            let new_number = number - 120;
            let var = list_of_persons.get(number as usize);
            if var != None {
                let p = var.unwrap().clone();
                list_of_persons_max3.push(p);
                ex_pers3.push(existing_pers.get(number as usize).unwrap().clone());
            }
        } else if number < 240 {
            let new_number = number - 180;
            let var = list_of_persons.get(number as usize);
            if var != None {
                let p = var.unwrap().clone();
                list_of_persons_max4.push(p);
                ex_pers4.push(existing_pers.get(number as usize).unwrap().clone());
            }
        } else if number < 300 {
            let new_number = number - 240;
            let var = list_of_persons.get(number as usize);
            if var != None {
                let p = var.unwrap().clone();
                list_of_persons_max5.push(p);
                ex_pers5.push(existing_pers.get(number as usize).unwrap().clone());
            }
        } else if number < 360 {
            let new_number = number - 300;
            let var = list_of_persons.get(number as usize);
            if var != None {
                let p = var.unwrap().clone();
                list_of_persons_max6.push(p);
                ex_pers6.push(existing_pers.get(number as usize).unwrap().clone());
            }
        }
    }
    println!("calc!");
    let mut persons = [
        list_of_persons_max1,
        list_of_persons_max2,
        list_of_persons_max3,
        list_of_persons_max4,
        list_of_persons_max5,
        list_of_persons_max6,
    ];
    let ex_persons = [ex_pers1, ex_pers2, ex_pers3, ex_pers4, ex_pers5, ex_pers6];
    for _t in 0..time_shifts {
        //let mut iter = list_of_persons.iter();
        // all_shifts[_t as usize] = build_group(list_of_persons, group_size);
        println!("shifts {}", _t);
        let return_groups_persons = build_group1(persons.clone(), group_size, ex_persons.clone());
        all_shifts[_t as usize] = return_groups_persons.0;
        persons = return_groups_persons.1;

        //build_groups1(iter.next(), iter.next(), iter.next(), iter.next());
    }
    // println!(" Persons {:?}", persons);
    all_shifts
}

fn build_group1(
    persons: [Vec<Persons>; 6],
    group_size: i32,
    possible_persons: [Vec<i32>; 6],
) -> (Vec<Group>, [Vec<Persons>; 6]) {
    println!("new");
    let mut t = String::new();
    let mut return_updated_persons = persons.clone();
    let mut overall_return = 0;
    let mut counter = 1;
    let mut pos_in_group = 0;
    //let perm = persons[0].get(51).unwrap().clone();

    //let mut persons_in_group = Vec::new();

    let mut sec_counter = 0;

    let mut group_list = Vec::new();

    //muss auch zurückgegeben werden

    //jeder zeitslot
    for pers in persons.clone() {
        t = t + ".";
        let mut updated_persons: Vec<Persons> = Vec::new();
        let mut tmp = pers.clone();
        //println!("{}", persons.len());
        //println!("new Personsgroup with {} length", pers.len());
        if (tmp.len() == 0) {
            println!("Stopped");
            break;
        }

        let mut index = tmp.len() - 1;
        let mut names = Vec::new();

        let mut pos_in_tmp = Vec::new();
        let mut current_person = tmp.get(index).unwrap().clone();
        let mut init_person = tmp.get(index - 1).unwrap().clone();
        //let mut init_person1 = tmp.get(index - 2).unwrap().clone();
        //let mut init_person2 = tmp.get(index - 3).unwrap().clone();
        let mut init_person1 = init_person.clone();
        let mut init_person2 = init_person1.clone();
        let mut persons_in_group = Vec::new();
        for i in 0..group_size {
            persons_in_group.push(current_person.clone());
        }
        //let mut persons_in_group: [Persons; group_size] =
        //    [current_person, init_person, init_person1, init_person2];
        while !tmp.is_empty() {
            // println!("Index is : {}", index);

            current_person = tmp.get(index).unwrap().clone();
            if (counter < 6) {
                println!("Current_person{}", current_person.name);
                println!("index {}", index);
            }

            //* Persons for Groups getting searched
            //Personen in der Gruppe 2/3/4
            //  println!("Counter {}", counter);
            //println!("Index {}",index);
            //println!("{:?}",current_person.peoplemet);
            //println!("{:?}",names);
            //println!("{:?}",current_person.name);
            //println!("{}", t);
            let peoplemet_set: HashSet<_> = current_person.peoplemet.iter().copied().collect();

            if names.iter().all(|item| !peoplemet_set.contains(item)) {
                println!("test");
                names.push(current_person.name);
                //println!("aktuelle Person {:?}", current_person.peoplemet);
                println!("pos:674: {}", pos_in_group);
                println!("Aktuelle Person ist: in 675: {}", current_person.name);
                persons_in_group[pos_in_group] = Persons {
                    name: current_person.name,
                    peoplemet: current_person.peoplemet,
                    number_of_meetings: current_person.number_of_meetings,
                };
                tmp.pop();
                pos_in_tmp.push(index);
                counter = counter + 1;
                if index == 0 {
                } else {
                    index = index - 1;
                }
                pos_in_group += 1;
            } else {
                //* Persons are getting searched, posibility of double Meetings
                t = t + "+";

                let result = get_new_person(current_person, tmp, names.clone());
                //println!("Run");
                current_person = result.0.get(result.0.len() - 1).unwrap().clone();
                persons_in_group[pos_in_group] = Persons {
                    name: current_person.name,
                    peoplemet: current_person.peoplemet,
                    number_of_meetings: current_person.number_of_meetings,
                };
                tmp = result.0;

                let mut pos_in_old_tmp = 0;
                let mut ref_pos = pos_in_tmp.clone();
                // println!("pos_in_tmp 466 {}", pos_in_tmp.len());

                for count in pos_in_tmp {
                    //println!("pos_in_tmp 466 {}", count);

                    ref_pos[pos_in_old_tmp] = ref_pos.get(pos_in_old_tmp).unwrap() - 1;
                    pos_in_old_tmp += 1;
                }
                pos_in_tmp = ref_pos;
                let pos = tmp.len() - 1;
                pos_in_tmp.push(pos);
                names.push(current_person.name);
                tmp.pop();
                if index == 0 {
                } else {
                    index = index - 1;
                }
                println!("pos:674: {}", pos_in_group);
                println!("Aktuelle Person ist: in 675: {}", current_person.name);

                pos_in_group += 1;
                counter += 1;
            }
            //Groups are getting created
            /* */
            if (counter - 1) % group_size == 0 {
                t = t + ".";
                let mut cur_pers = persons_in_group
                    .get(persons_in_group.len() - 1)
                    .unwrap()
                    .clone();
                let peoplemet_set: HashSet<_> = cur_pers.peoplemet.iter().copied().collect();
                // new_names = names.clone();
                if names.iter().all(|item| !peoplemet_set.contains(item)) {
                    //*people didn't met eachother yet, Groups get created
                    let mut p_help = Vec::new();

                    for mut person in persons_in_group.clone() {
                        for name in names.clone() {
                            if (name == person.name) {
                            } else {
                                person.peoplemet.push(name);
                                person.number_of_meetings.push(1);
                            }
                        }
                        let p = person.clone();
                        updated_persons.push(person);
                        p_help.push(p);
                    }
                    let t = p_help.clone();
                    let new_group = Group {
                        group_size: group_size,
                        persons_in_group: p_help.to_vec(),
                    };
                    group_list.push(new_group);

                    pos_in_group = 0;
                    sec_counter += 1;
                    pos_in_tmp.push(index);
                    pos_in_tmp.sort();
                    names = Vec::new();
                    //let mut len = tmp.len() - 1;

                    if index == 0 {
                        break;
                    } else {
                    }
                } else {
                    //*Group gets created but with people who have met each other already */
                    let mut p_help = Vec::new();

                    for mut person in persons_in_group.clone() {
                        for name in names.clone() {
                            if name == person.name {
                            } else {
                                if person.peoplemet.contains(&name) {
                                    let pos_sec =
                                        person.peoplemet.iter().position(|&r| r == name).unwrap();
                                    person.number_of_meetings[pos_sec] =
                                        person.number_of_meetings.get(pos_sec).unwrap().clone() + 1;
                                } else {
                                    person.peoplemet.push(name);
                                    person.number_of_meetings.push(1);
                                }
                            }
                        }
                        let p = person.clone();
                        updated_persons.push(person);
                        p_help.push(p);
                    }

                    let new_group = Group {
                        group_size: group_size,
                        persons_in_group: p_help.to_vec(),
                    };
                    group_list.push(new_group);
                    names = Vec::new();
                    pos_in_group = 0;
                    sec_counter += 1;
                    pos_in_tmp.push(index);
                    pos_in_tmp.sort();
                    // println!("{:?}", pos_in_tmp);
                    // let mut len = tmp.len() - 1;

                    if index == 0 {
                        break;
                    } else {
                    }

                    //implement what do do if people know each other
                    //TODO - increment of the meteachother -param, done by lines 387 -563
                    t = t + "/";
                    //println!("{}", t);
                    //let result = get_new_person(current_person, tmp, names.clone());
                    //current_person = result.0.get(result.0.len() - 1).unwrap().clone();

                    //tmp = result.0;
                    //* Wenn drei mal geentert wird wird dreimal die 51 gepusht, weil es dann immer das letzte element ist */
                    //let mut pos_in_old_tmp = 0;
                    //let mut ref_pos = pos_in_tmp.clone();
                    /*for count in pos_in_tmp {
                        //println!("pos_in_tmp 409 {}", count);

                        ref_pos[pos_in_old_tmp] = ref_pos.get(pos_in_old_tmp).unwrap() - 1;
                        pos_in_old_tmp += 1;
                    }
                    pos_in_tmp = ref_pos;
                    let pos = tmp.len() - 1;

                    pos_in_tmp.push(pos);
                    names.push(current_person.name);
                    if index == 0 {
                        break;
                    } else {
                        index = index - 1;
                    }*/
                    //pos_in_group += 1;
                }
            } else {
                if tmp.is_empty() {
                    let mut cur_pers = persons_in_group
                        .get(persons_in_group.len() - 1)
                        .unwrap()
                        .clone();
                    let peoplemet_set: HashSet<_> = cur_pers.peoplemet.iter().copied().collect();
                    // new_names = names.clone();
                    if names.iter().all(|item| !peoplemet_set.contains(item)) {
                        //*people didn't met eachother yet, Groups get created
                        // let fourth_pers = names.pop().unwrap();
                        let third_pers = names.pop().unwrap();
                        let second_pers = names.pop().unwrap();
                        let first_pers = names.pop().unwrap();

                        //let mut p1 = persons_in_group.get(0).unwrap().clone();
                        //let mut p2 = persons_in_group.get(1).unwrap().clone();
                        //                        let mut p3 = persons_in_group.get(0).unwrap().clone();

                        cur_pers.peoplemet.push(first_pers);
                        cur_pers.peoplemet.push(second_pers);
                        //cur_pers.peoplemet.push(third_pers);
                        cur_pers.number_of_meetings.push(1);
                        cur_pers.number_of_meetings.push(1);
                        // cur_pers.number_of_meetings.push(1);
                        //*Teilnehmer 3
                        persons_in_group[2] = Persons {
                            name: cur_pers.name,
                            peoplemet: cur_pers.clone().peoplemet,
                            number_of_meetings: cur_pers.clone().number_of_meetings,
                        };
                        //*Teilnehmer 1
                        let mut p0 = persons_in_group.get(0).unwrap().clone();
                        p0.peoplemet.push(second_pers);
                        //p0.peoplemet.push(third_pers);
                        p0.peoplemet.push(cur_pers.name);
                        p0.number_of_meetings.push(1);
                        // p0.number_of_meetings.push(1);
                        p0.number_of_meetings.push(1);
                        //*Teilnehmer 2
                        let mut p1 = persons_in_group.get(1).unwrap().clone();
                        p1.peoplemet.push(first_pers);
                        //p1.peoplemet.push(third_pers);
                        p1.peoplemet.push(cur_pers.name);
                        p1.number_of_meetings.push(1);
                        //p1.number_of_meetings.push(1);
                        p1.number_of_meetings.push(1);

                        //*Teilnehmer 3
                        /*let mut p2 = persons_in_group.get(2).unwrap().clone();
                        p2.peoplemet.push(first_pers);
                        p2.peoplemet.push(second_pers);
                        p2.peoplemet.push(cur_pers.name);
                        p2.number_of_meetings.push(1);
                        p2.number_of_meetings.push(1);
                        p2.number_of_meetings.push(1);*/

                        persons_in_group[0] = p0;
                        persons_in_group[1] = p1;
                        // persons_in_group[2] = p2;
                        updated_persons.push(persons_in_group.get(0).unwrap().clone());
                        updated_persons.push(persons_in_group.get(1).unwrap().clone());
                        updated_persons.push(persons_in_group.get(2).unwrap().clone());
                        //updated_persons.push(persons_in_group.get(3).unwrap().clone());
                        let new_group = Group {
                            group_size: 4,
                            persons_in_group: persons_in_group.to_vec(),
                        };
                        group_list.push(new_group);

                        // let new_perm = persons[0].get(51).unwrap().clone();

                        //persons_in_group = Vec::new();
                        let mut tmp_ind = index;
                        if (index == 0) {
                            tmp_ind = index;
                        } else {
                            tmp_ind = index - 1;
                        }
                        //let new_perm = tmp.get(tmp_ind).unwrap().clone();

                        //persons_in_group = vec![new_perm; 4];
                        //counter += 1;
                        pos_in_group = 0;
                        sec_counter += 1;
                        pos_in_tmp.push(index);
                        pos_in_tmp.sort();

                        let mut len = tmp.len() - 1;
                        //println!("davor{}", pos_in_tmp.get(pos_in_tmp.len() - 1).unwrap());
                        // println!("danach{}", len);

                        if index == 0 {
                            break;
                        } else {
                        }
                    } else {
                        //*Group gets created but with people who have met each other already */
                        let third_pers = names.pop().unwrap();
                        let second_pers = names.pop().unwrap();
                        let first_pers = names.pop().unwrap();

                        /*  if peoplemet_set.contains(&third_pers) {
                            let position_person3 = cur_pers
                                .peoplemet
                                .iter()
                                .position(|&r| r == third_pers)
                                .unwrap();

                            cur_pers.number_of_meetings[position_person3] = cur_pers
                                .number_of_meetings
                                .get(position_person3)
                                .unwrap()
                                .clone()
                                + 1;
                        } else {
                            cur_pers.peoplemet.push(third_pers);
                            cur_pers.number_of_meetings.push(1);
                        }*/

                        if peoplemet_set.contains(&second_pers) {
                            let position_person2 = cur_pers
                                .peoplemet
                                .iter()
                                .position(|&r| r == second_pers)
                                .unwrap();
                            cur_pers.number_of_meetings[position_person2] = cur_pers
                                .number_of_meetings
                                .get(position_person2)
                                .unwrap()
                                .clone()
                                + 1;
                        } else {
                            cur_pers.peoplemet.push(second_pers);
                            cur_pers.number_of_meetings.push(1);
                        }

                        if peoplemet_set.contains(&first_pers) {
                            let position_person1 = cur_pers
                                .peoplemet
                                .iter()
                                .position(|&r| r == first_pers)
                                .unwrap();

                            cur_pers.number_of_meetings[position_person1] = cur_pers
                                .number_of_meetings
                                .get(position_person1)
                                .unwrap()
                                .clone()
                                + 1;
                        } else {
                            cur_pers.peoplemet.push(first_pers);
                            cur_pers.number_of_meetings.push(1);
                        }
                        println!("Aktuelle Person ist: in 468: {}", cur_pers.name);
                        persons_in_group[2] = Persons {
                            name: cur_pers.name,
                            peoplemet: cur_pers.clone().peoplemet,
                            number_of_meetings: cur_pers.clone().number_of_meetings,
                        };

                        //*Teilnehmer 1
                        let mut p0 = persons_in_group.get(0).unwrap().clone();
                        if p0.peoplemet.contains(&second_pers) {
                            let pos_sec =
                                p0.peoplemet.iter().position(|&r| r == second_pers).unwrap();
                            p0.number_of_meetings[pos_sec] =
                                p0.number_of_meetings.get(pos_sec).unwrap().clone() + 1;
                        } else {
                            p0.peoplemet.push(second_pers);
                            p0.number_of_meetings.push(1);
                        }
                        /*  if p0.peoplemet.contains(&third_pers) {
                            let pos_th =
                                p0.peoplemet.iter().position(|&r| r == third_pers).unwrap();
                            p0.number_of_meetings[pos_th] =
                                p0.number_of_meetings.get(pos_th).unwrap().clone() + 1;
                        } else {
                            p0.peoplemet.push(third_pers);
                            p0.number_of_meetings.push(1);
                        }*/
                        if p0.peoplemet.contains(&current_person.name) {
                            let pos_cur = p0
                                .peoplemet
                                .iter()
                                .position(|&r| r == current_person.name)
                                .unwrap();
                            p0.number_of_meetings[pos_cur] =
                                p0.number_of_meetings.get(pos_cur).unwrap().clone() + 1;
                        } else {
                            p0.peoplemet.push(current_person.name);
                            p0.number_of_meetings.push(1);
                        }

                        persons_in_group[0] = p0;

                        //*Teilnehmer 2
                        let mut p1 = persons_in_group.get(1).unwrap().clone();
                        if p1.peoplemet.contains(&first_pers) {
                            let pos_sec =
                                p1.peoplemet.iter().position(|&r| r == first_pers).unwrap();
                            p1.number_of_meetings[pos_sec] =
                                p1.number_of_meetings.get(pos_sec).unwrap().clone() + 1;
                        } else {
                            p1.peoplemet.push(first_pers);
                            p1.number_of_meetings.push(1);
                        }
                        /*  if p1.peoplemet.contains(&third_pers) {
                            let pos_th =
                                p1.peoplemet.iter().position(|&r| r == third_pers).unwrap();
                            p1.number_of_meetings[pos_th] =
                                p1.number_of_meetings.get(pos_th).unwrap().clone() + 1;
                        } else {
                            p1.peoplemet.push(third_pers);
                            p1.number_of_meetings.push(1);
                        }*/
                        if p1.peoplemet.contains(&current_person.name) {
                            let pos_cur = p1
                                .peoplemet
                                .iter()
                                .position(|&r| r == current_person.name)
                                .unwrap();
                            p1.number_of_meetings[pos_cur] =
                                p1.number_of_meetings.get(pos_cur).unwrap().clone() + 1;
                        } else {
                            p1.peoplemet.push(current_person.name);
                            p1.number_of_meetings.push(1);
                        }

                        persons_in_group[1] = p1;

                        //*Teilnehmer 3
                        /*let mut p2 = persons_in_group.get(2).unwrap().clone();
                        if p2.peoplemet.contains(&second_pers) {
                            let pos_sec =
                                p2.peoplemet.iter().position(|&r| r == second_pers).unwrap();
                            p2.number_of_meetings[pos_sec] =
                                p2.number_of_meetings.get(pos_sec).unwrap().clone() + 1;
                        } else {
                            p2.peoplemet.push(second_pers);
                            p2.number_of_meetings.push(1);
                        }
                        if p2.peoplemet.contains(&third_pers) {
                            let pos_th =
                                p2.peoplemet.iter().position(|&r| r == third_pers).unwrap();
                            p2.number_of_meetings[pos_th] =
                                p2.number_of_meetings.get(pos_th).unwrap().clone() + 1;
                        } else {
                            p2.peoplemet.push(third_pers);
                            p2.number_of_meetings.push(1);
                        }
                        if p2.peoplemet.contains(&current_person.name) {
                            let pos_cur = p2
                                .peoplemet
                                .iter()
                                .position(|&r| r == current_person.name)
                                .unwrap();
                            p2.number_of_meetings[pos_cur] =
                                p2.number_of_meetings.get(pos_cur).unwrap().clone() + 1;
                        } else {
                            p2.peoplemet.push(current_person.name);
                            p2.number_of_meetings.push(1);
                        }

                        persons_in_group[2] = p2;*/

                        //*The 4 Members of the Groups are created, groups will be created now  */
                        updated_persons.push(persons_in_group.get(0).unwrap().clone());
                        updated_persons.push(persons_in_group.get(1).unwrap().clone());
                        updated_persons.push(persons_in_group.get(2).unwrap().clone());
                        //updated_persons.push(persons_in_group.get(3).unwrap().clone());
                        let new_group = Group {
                            group_size: 4,
                            persons_in_group: persons_in_group.to_vec(),
                        };
                        group_list.push(new_group);

                        let mut tmp_ind = index;
                        if (index == 0) {
                            tmp_ind = index;
                        } else {
                            tmp_ind = index - 1;
                        }
                        //let new_perm = tmp.get(tmp_ind).unwrap().clone();

                        //persons_in_group = vec![new_perm; 4];
                        //counter += 1;
                        pos_in_group = 0;
                        sec_counter += 1;
                        pos_in_tmp.push(index);
                        pos_in_tmp.sort();
                        // println!("{:?}", pos_in_tmp);
                        let mut len = tmp.len() - 1;

                        if index == 0 {
                            break;
                        } else {
                        }
                    }
                }
            }
        }
        return_updated_persons[overall_return] = updated_persons;
        overall_return = overall_return + 1;
        println!("one Persongroup finished");
        pos_in_group = 0;

        counter = 1;
    }

    println!("one Timeslot finished");

    return (group_list, return_updated_persons);
    //group_list
}

fn get_new_person(
    old_person: Persons,
    possible_persons: Vec<Persons>,
    group: Vec<i32>,
) -> (Vec<Persons>, usize) {
    //save new Person on the top of the stack
    let mut result_persons: Vec<Persons> = Vec::new();
    let mut hit = false;
    let mut new_person;
    let mut maxMeetings = 2;
    let tooMuchMeetings = 5;

    let mut index = 0;
    let mut return_index = 0;
    while !hit {
        println!("length{}", possible_persons.len());
        println!("index{}", index);
        new_person = possible_persons.get(index).unwrap().clone();
        println!("----------------------------");

        if new_person == old_person {
            //println!("t");
            if index == possible_persons.len() - 1 {
                println!("537");
                //keine unknows findbar => Dopplungen
                //index = 0;
                return_index = index;

                result_persons = possible_persons.clone();
                //letzte stelle ist jetzt die neue Person
                result_persons.swap(index, possible_persons.len() - 1);
                hit = true;
            }
            index += 1;

            println!("lul");
            continue;
        } else {
            let peoplemet_set: HashSet<_> = new_person.peoplemet.iter().copied().collect();
            if group.iter().all(|item| !peoplemet_set.contains(item)) {
                result_persons = possible_persons.clone();
                //letzte stelle ist jetzt die neue Person
                result_persons.swap(index, possible_persons.len() - 1);
                hit = true;
                return_index = index;
                println!("Run");
            } else {
                println!("Index {}", index);
                //println!("{}", possible_persons.len());
                //println!("{}", hit);
                if index == possible_persons.len() - 1 {
                    println!("537");
                    //keine unknows findbar => Dopplungen
                    index = 0;

                    let mut intern_count = true;
                    while intern_count {
                        new_person = possible_persons.get(index).unwrap().clone();
                        println!("run intern");
                        let mut is_good = false;
                        let peoplemet_set: HashSet<_> =
                            new_person.peoplemet.iter().copied().collect();
                        let peoplcecount_set: HashSet<_> =
                            new_person.number_of_meetings.iter().copied().collect();

                        let mut position = 0;
                        let groupClone = group.clone();
                        for i in groupClone {
                            position = peoplemet_set.iter().position(|&r| r == i).unwrap();
                            if peoplcecount_set.get(&(position as i32)).unwrap().clone()
                                >= maxMeetings
                            {
                                is_good = false;
                            } else {
                                is_good = true;
                            }
                        }

                        if is_good {
                            result_persons = possible_persons.clone();
                            //letzte stelle ist jetzt die neue Person
                            result_persons.swap(index, possible_persons.len() - 1);
                            hit = true;
                            return_index = index;
                            intern_count = false;
                        }

                        if index == possible_persons.len() - 1 {
                            maxMeetings += 1;
                            println!("Max Meetings: {}", maxMeetings);
                            if tooMuchMeetings == maxMeetings {
                                println!("To much meetings");
                                exit(0);
                            }
                            index = 0;
                        } else {
                            index += 1;
                        }
                    }
                } else {
                    index = index + 1;
                }
            }
        }
    }

    return (result_persons, return_index);
}

fn build_file(slots: Vec<Vec<Group>>, filename: String, quality: [f64; 7]) -> Vec<u8> {
    let workbook = Workbook::new(&filename);

    let mut format1 = workbook.add_format().set_font_color(FormatColor::Red);

    let mut format2 = workbook
        .add_format()
        .set_font_color(FormatColor::Blue)
        .set_underline(FormatUnderline::Single);

    let mut format3 = workbook
        .add_format()
        .set_font_color(FormatColor::Green)
        .set_align(FormatAlignment::CenterAcross)
        .set_align(FormatAlignment::VerticalCenter);

    let mut sheet1 = workbook
        .add_worksheet(None)
        .expect("Error in Workbook Creation");
    let mut line_number = 0;
    let mut field_number = 0;
    for slot in slots {
        for group in slot {
            for person in group.persons_in_group {
                sheet1.write_number(line_number, field_number, person.name.into(), None);
                field_number += 1;
            }
            field_number += 1;
        }
        line_number += 1;
        field_number = 0;
    }
    line_number += 1;
    //let mut summed_quality = 0.0;
    let mut qua = String::new();
    let mut count = 0;
    for q in quality {
        if count == 0 {
            qua = "Einfache Treffen".to_string();
        } else if count == 1 {
            qua = "Double-Anzahl".to_string();
        } else if count == 2 {
            qua = "Trible-Anzahl".to_string();
        } else if count == 3 {
            qua = "Quatruple-Anzahl".to_string()
        } else if count == 4 {
            qua = "Gesamtqualität".to_string();
        } else if count == 5 {
            qua = "Person mit höchster Anzahl".to_string();
        } else if count == 6 {
            qua = "Höchste Anzahl".to_string();
        }

        sheet1.write_string(line_number, field_number, &qua, None);
        field_number += 1;
        sheet1.write_string(line_number, field_number, &q.to_string(), None);
        field_number += 1;

        count += 1;
    }
    //sheet1.write_string(line_number, field_number, "Gesamtqualität", None);
    //field_number += 1;
    //sheet1.write_string(line_number, field_number, &summed_quality.to_string(), None);

    sheet1.set_selection(1, 0, 1, 2);
    sheet1.set_tab_color(FormatColor::Cyan);
    workbook.close().expect("Closing Error");

    let result = fs::read(&filename).expect("can read file");
    result
}

fn calculateQuality(
    valueDobble: f64,
    valueTrible: f64,
    valueQuatruple: f64,
    slots: Vec<Vec<Group>>,
) -> [f64; 7] {
    let mut sing = 0.0;
    let mut dub = 0.0;
    let mut tri = 0.0;
    let mut qua = 0.0;
    let mut high = 0.0;
    let mut p = 0;
    let mut calculatedArray = [0.0; 7];
    for slot in slots {
        for group in slot {
            for person in group.persons_in_group {
                for met in person.number_of_meetings {
                    if met > high as i32 {
                        high = met as f64;

                        p = person.name;
                    }
                    if met == 1 {
                        sing += 1.0;
                    }
                    if (met == 2) {
                        dub += 1.0;
                    }
                    if (met == 3) {
                        tri += 1.0;
                    }
                    if (met == 4) {
                        qua += 1.0;
                    }
                }
            }
        }
    }

    println!("dub {}", dub);
    println!("tri {}", tri);
    println!("qua {}", qua);

    calculatedArray[1] = dub;
    calculatedArray[2] = tri;
    calculatedArray[3] = qua;
    calculatedArray[0] = sing;
    calculatedArray[4] = dub * valueDobble + tri * valueTrible + qua * valueQuatruple; //Gesamtqualität
    calculatedArray[5] = p as f64;
    calculatedArray[6] = high;
    println!("calculatedArray {:?}", calculatedArray);
    calculatedArray
}

fn try_better_result2(
    input: Vec<Vec<Group>>,
    group_size: i32,
    time_shifts: i32,
) -> Vec<Vec<Group>> {
    let mut rng = rand::thread_rng();

    let mut output = input.clone();

    let mut counter = 0;
    for mut timeslots in input {
        let group_count = timeslots.len(); //Group count ist die Anzahl an Gruppen
        let first_group_number = rng.gen_range(0..group_count); // 0 bis Group-count -1
        let mut good_number = true;
        let mut second_group_number = rng.gen_range(0..group_count);
        while good_number {
            if (second_group_number != first_group_number) {
                good_number = false;
            } else {
                second_group_number = rng.gen_range(0..group_count);
            }
        }
        let mut first_group = timeslots.get(first_group_number).unwrap().clone();
        let mut second_group = timeslots.get(second_group_number).unwrap().clone();
        let first_member = rng.gen_range(0..group_size);
        let sec_member = rng.gen_range(0..group_size);

        let person1 = first_group
            .persons_in_group
            .get(first_member as usize)
            .unwrap()
            .clone();
        let person2 = second_group
            .persons_in_group
            .get(sec_member as usize)
            .unwrap()
            .clone();

        first_group
            .persons_in_group
            .swap(first_member as usize, (group_size - 1) as usize);

        first_group.persons_in_group.pop();
        first_group.persons_in_group.push(person2);

        second_group
            .persons_in_group
            .swap(sec_member as usize, (group_size - 1) as usize);

        second_group.persons_in_group.pop();
        second_group.persons_in_group.push(person1);

        timeslots.swap(first_group_number as usize, (group_count - 1) as usize);
        timeslots.pop();
        timeslots.push(first_group);
        if (second_group_number == group_count - 1) {
            second_group_number = first_group_number;
        }
        timeslots.swap(second_group_number, (group_count - 1) as usize);
        timeslots.pop();
        timeslots.push(second_group);

        output.swap(counter, (time_shifts - 1) as usize);
        output.pop();
        output.push(timeslots);
        output.swap(counter, (time_shifts - 1) as usize);
        counter += 1;
    }
    output
}

fn try_better_result3(
    input: Vec<Vec<Group>>,
    group_size: i32,
    time_shifts: i32,
) -> Vec<Vec<Group>> {
    let mut control = Vec::new();
    let mut rng = rand::thread_rng();

    let mut output = input.clone();

    let mut counter = 0;
    for mut timeslots in input {
        let group_count = timeslots.len(); //Group count ist die Anzahl an Gruppen
        let first_group_number = rng.gen_range(0..group_count); // 0 bis Group-count -1
        let mut sec_number = true;
        let mut third_number = true;
        let mut second_group_number = rng.gen_range(0..group_count);
        let mut third_group_number = rng.gen_range(0..group_count);
        while sec_number || third_number {
            if (second_group_number != first_group_number
                && second_group_number != third_group_number)
            {
                sec_number = false;
            } else {
                second_group_number = rng.gen_range(0..group_count);
            }
            if (third_group_number != first_group_number
                && third_group_number != second_group_number)
            {
                third_number = false;
            } else {
                third_group_number = rng.gen_range(0..group_count);
            }
        }
        let mut first_group = timeslots.get(first_group_number).unwrap().clone();
        let mut second_group = timeslots.get(second_group_number).unwrap().clone();
        let mut third_group = timeslots.get(third_group_number).unwrap().clone();
        let first_member = rng.gen_range(0..group_size);
        let sec_member = rng.gen_range(0..group_size);
        let third_member = rng.gen_range(0..group_size);

        let person1 = first_group
            .persons_in_group
            .get(first_member as usize)
            .unwrap()
            .clone();
        let person2 = second_group
            .persons_in_group
            .get(sec_member as usize)
            .unwrap()
            .clone();
        let person3 = third_group
            .persons_in_group
            .get(third_member as usize)
            .unwrap()
            .clone();
        control.push(person1.clone());
        control.push(person2.clone());
        control.push(person3.clone());

        first_group
            .persons_in_group
            .swap(first_member as usize, (group_size - 1) as usize);

        first_group.persons_in_group.pop();
        first_group.persons_in_group.push(person3);

        second_group
            .persons_in_group
            .swap(sec_member as usize, (group_size - 1) as usize);

        second_group.persons_in_group.pop();
        second_group.persons_in_group.push(person1);

        third_group
            .persons_in_group
            .swap(third_member as usize, (group_size - 1) as usize);

        third_group.persons_in_group.pop();
        third_group.persons_in_group.push(person2);
        timeslots.swap(first_group_number as usize, (group_count - 1) as usize);
        timeslots.pop();
        timeslots.push(first_group);
        timeslots.swap(first_group_number as usize, (group_count - 1) as usize);

        timeslots.swap(second_group_number, (group_count - 1) as usize);
        timeslots.pop();
        timeslots.push(second_group);
        timeslots.swap(second_group_number, (group_count - 1) as usize);

        timeslots.swap(third_group_number, (group_count - 1) as usize);
        timeslots.pop();
        timeslots.push(third_group);
        timeslots.swap(third_group_number, (group_count - 1) as usize);

        output.swap(counter, (time_shifts - 1) as usize);
        output.pop();
        output.push(timeslots);
        output.swap(counter, (time_shifts - 1) as usize);
        counter += 1;
    }
    output
}

fn try_better_result4(
    input: Vec<Vec<Group>>,
    group_size: i32,
    time_shifts: i32,
) -> Vec<Vec<Group>> {
    let mut control = Vec::new();
    let mut rng = rand::thread_rng();

    let mut output = input.clone();

    let mut counter = 0;
    for mut timeslots in input {
        let group_count = timeslots.len(); //Group count ist die Anzahl an Gruppen
        let first_group_number = rng.gen_range(0..group_count); // 0 bis Group-count -1
        let mut sec_number = true;
        let mut third_number = true;
        let mut fourth_number = true;
        let mut second_group_number = rng.gen_range(0..group_count);
        let mut third_group_number = rng.gen_range(0..group_count);
        let mut fourth_group_number = rng.gen_range(0..group_count);
        while sec_number || third_number || fourth_number {
            if (second_group_number != first_group_number
                && second_group_number != third_group_number
                && second_group_number != fourth_group_number)
            {
                sec_number = false;
            } else {
                sec_number = true;
                second_group_number = rng.gen_range(0..group_count);
            }
            if (third_group_number != first_group_number
                && third_group_number != second_group_number
                && third_group_number != fourth_group_number)
            {
                third_number = false;
            } else {
                third_number = true;
                third_group_number = rng.gen_range(0..group_count);
            }
            if (fourth_group_number != first_group_number
                && fourth_group_number != third_group_number
                && fourth_group_number != second_group_number)
            {
                fourth_number = false;
            } else {
                fourth_number = true;
                fourth_group_number = rng.gen_range(0..group_count);
            }
        }
        let mut first_group = timeslots.get(first_group_number).unwrap().clone();
        let mut second_group = timeslots.get(second_group_number).unwrap().clone();
        let mut third_group = timeslots.get(third_group_number).unwrap().clone();
        let mut fourth_group = timeslots.get(fourth_group_number).unwrap().clone();
        let first_member = rng.gen_range(0..group_size);
        let sec_member = rng.gen_range(0..group_size);
        let third_member = rng.gen_range(0..group_size);
        let fourth_member = rng.gen_range(0..group_size);

        let person1 = first_group
            .persons_in_group
            .get(first_member as usize)
            .unwrap()
            .clone();
        let person2 = second_group
            .persons_in_group
            .get(sec_member as usize)
            .unwrap()
            .clone();
        let person3 = third_group
            .persons_in_group
            .get(third_member as usize)
            .unwrap()
            .clone();
        let person4 = fourth_group
            .persons_in_group
            .get(fourth_member as usize)
            .unwrap()
            .clone();
        control.push(person1.clone());
        control.push(person2.clone());
        control.push(person3.clone());
        control.push(person4.clone());
        first_group
            .persons_in_group
            .swap(first_member as usize, (group_size - 1) as usize);

        first_group.persons_in_group.pop();
        first_group.persons_in_group.push(person4);

        second_group
            .persons_in_group
            .swap(sec_member as usize, (group_size - 1) as usize);

        second_group.persons_in_group.pop();
        second_group.persons_in_group.push(person1);

        third_group
            .persons_in_group
            .swap(third_member as usize, (group_size - 1) as usize);

        third_group.persons_in_group.pop();
        third_group.persons_in_group.push(person2);

        fourth_group
            .persons_in_group
            .swap(fourth_member as usize, (group_size - 1) as usize);

        fourth_group.persons_in_group.pop();
        fourth_group.persons_in_group.push(person3);

        timeslots.swap(first_group_number as usize, (group_count - 1) as usize);
        timeslots.pop();
        timeslots.push(first_group);
        timeslots.swap(first_group_number as usize, (group_count - 1) as usize);

        timeslots.swap(second_group_number, (group_count - 1) as usize);
        timeslots.pop();
        timeslots.push(second_group);
        timeslots.swap(second_group_number, (group_count - 1) as usize);

        timeslots.swap(third_group_number, (group_count - 1) as usize);
        timeslots.pop();
        timeslots.push(third_group);
        timeslots.swap(third_group_number, (group_count - 1) as usize);

        timeslots.swap(fourth_group_number, (group_count - 1) as usize);
        timeslots.pop();
        timeslots.push(fourth_group);
        timeslots.swap(fourth_group_number, (group_count - 1) as usize);

        output.swap(counter, (time_shifts - 1) as usize);
        output.pop();
        output.push(timeslots);
        output.swap(counter, (time_shifts - 1) as usize);
        counter += 1;
    }
    output
}

fn try_better_result8(
    input: Vec<Vec<Group>>,
    group_size: i32,
    time_shifts: i32,
) -> Vec<Vec<Group>> {
    let mut control = Vec::new();
    let mut rng = rand::thread_rng();

    let mut output = input.clone();

    let mut counter = 0;
    for mut timeslots in input {
        let group_count = timeslots.len(); //Group count ist die Anzahl an Gruppen
        let first_group_number = rng.gen_range(0..group_count); // 0 bis Group-count -1
        let mut sec_number = true;
        let mut third_number = true;
        let mut fourth_number = true;
        let mut five_number = true;
        let mut six_number = true;
        let mut seven_number = true;
        let mut eight_number = true;

        let mut second_group_number = rng.gen_range(0..group_count);
        let mut third_group_number = rng.gen_range(0..group_count);
        let mut fourth_group_number = rng.gen_range(0..group_count);
        let mut five_group_number = rng.gen_range(0..group_count);
        let mut six_group_number = rng.gen_range(0..group_count);
        let mut seven_group_number = rng.gen_range(0..group_count);
        let mut eight_group_number = rng.gen_range(0..group_count);

        while sec_number
            || third_number
            || fourth_number
            || five_number
            || six_number
            || seven_number
            || eight_number
        {
            if (second_group_number != first_group_number
                && second_group_number != third_group_number
                && second_group_number != fourth_group_number
                && second_group_number != five_group_number
                && second_group_number != six_group_number
                && second_group_number != seven_group_number
                && second_group_number != eight_group_number)
            {
                sec_number = false;
            } else {
                second_group_number = rng.gen_range(0..group_count);
            }
            if (third_group_number != first_group_number
                && third_group_number != second_group_number
                && third_group_number != fourth_group_number
                && third_group_number != five_group_number
                && third_group_number != six_group_number
                && third_group_number != seven_group_number
                && third_group_number != eight_group_number)
            {
                third_number = false;
            } else {
                third_group_number = rng.gen_range(0..group_count);
            }
            if (fourth_group_number != first_group_number
                && fourth_group_number != third_group_number
                && fourth_group_number != second_group_number
                && fourth_group_number != five_group_number
                && fourth_group_number != six_group_number
                && fourth_group_number != seven_group_number
                && fourth_group_number != eight_group_number)
            {
                fourth_number = false;
            } else {
                fourth_group_number = rng.gen_range(0..group_count);
            }
            if (five_group_number != first_group_number
                && five_group_number != second_group_number
                && five_group_number != third_group_number
                && five_group_number != fourth_group_number
                && five_group_number != six_group_number
                && five_group_number != seven_group_number
                && five_group_number != eight_group_number)
            {
                five_number = false;
            } else {
                five_group_number = rng.gen_range(0..group_count);
            }
            if (six_group_number != first_group_number
                && six_group_number != second_group_number
                && six_group_number != third_group_number
                && six_group_number != fourth_group_number
                && six_group_number != five_group_number
                && six_group_number != seven_group_number
                && six_group_number != eight_group_number)
            {
                six_number = false;
            } else {
                six_group_number = rng.gen_range(0..group_count);
            }
            if (seven_group_number != first_group_number
                && seven_group_number != second_group_number
                && seven_group_number != third_group_number
                && seven_group_number != fourth_group_number
                && seven_group_number != six_group_number
                && seven_group_number != five_group_number
                && seven_group_number != eight_group_number)
            {
                seven_number = false;
            } else {
                seven_group_number = rng.gen_range(0..group_count);
            }
            if (eight_group_number != first_group_number
                && eight_group_number != second_group_number
                && eight_group_number != third_group_number
                && eight_group_number != fourth_group_number
                && eight_group_number != six_group_number
                && eight_group_number != seven_group_number
                && eight_group_number != five_group_number)
            {
                eight_number = false;
            } else {
                eight_group_number = rng.gen_range(0..group_count);
            }
        }
        let mut first_group = timeslots.get(first_group_number).unwrap().clone();
        let mut second_group = timeslots.get(second_group_number).unwrap().clone();
        let mut third_group = timeslots.get(third_group_number).unwrap().clone();
        let mut fourth_group = timeslots.get(fourth_group_number).unwrap().clone();
        let mut five_group = timeslots.get(five_group_number).unwrap().clone();
        let mut six_group = timeslots.get(six_group_number).unwrap().clone();
        let mut seven_group = timeslots.get(seven_group_number).unwrap().clone();
        let mut eight_group = timeslots.get(eight_group_number).unwrap().clone();

        let first_member = rng.gen_range(0..group_size);
        let sec_member = rng.gen_range(0..group_size);
        let third_member = rng.gen_range(0..group_size);
        let fourth_member = rng.gen_range(0..group_size);
        let five_member = rng.gen_range(0..group_size);
        let six_member = rng.gen_range(0..group_size);
        let seven_member = rng.gen_range(0..group_size);
        let eight_member = rng.gen_range(0..group_size);

        let person1 = first_group
            .persons_in_group
            .get(first_member as usize)
            .unwrap()
            .clone();
        let person2 = second_group
            .persons_in_group
            .get(sec_member as usize)
            .unwrap()
            .clone();
        let person3 = third_group
            .persons_in_group
            .get(third_member as usize)
            .unwrap()
            .clone();
        let person4 = fourth_group
            .persons_in_group
            .get(fourth_member as usize)
            .unwrap()
            .clone();

        let person5 = five_group
            .persons_in_group
            .get(five_member as usize)
            .unwrap()
            .clone();
        let person6 = six_group
            .persons_in_group
            .get(six_member as usize)
            .unwrap()
            .clone();
        let person7 = seven_group
            .persons_in_group
            .get(seven_member as usize)
            .unwrap()
            .clone();
        let person8 = eight_group
            .persons_in_group
            .get(eight_member as usize)
            .unwrap()
            .clone();

        control.push(person1.clone());
        control.push(person2.clone());
        control.push(person3.clone());
        control.push(person4.clone());

        control.push(person5.clone());
        control.push(person6.clone());
        control.push(person7.clone());
        control.push(person8.clone());
        first_group
            .persons_in_group
            .swap(first_member as usize, (group_size - 1) as usize);

        first_group.persons_in_group.pop();
        first_group.persons_in_group.push(person2);

        second_group
            .persons_in_group
            .swap(sec_member as usize, (group_size - 1) as usize);

        second_group.persons_in_group.pop();
        second_group.persons_in_group.push(person1);

        third_group
            .persons_in_group
            .swap(third_member as usize, (group_size - 1) as usize);

        third_group.persons_in_group.pop();
        third_group.persons_in_group.push(person4);

        fourth_group
            .persons_in_group
            .swap(fourth_member as usize, (group_size - 1) as usize);

        fourth_group.persons_in_group.pop();
        fourth_group.persons_in_group.push(person3);

        five_group
            .persons_in_group
            .swap(five_member as usize, (group_size - 1) as usize);

        five_group.persons_in_group.pop();
        five_group.persons_in_group.push(person6);

        six_group
            .persons_in_group
            .swap(six_member as usize, (group_size - 1) as usize);

        six_group.persons_in_group.pop();
        six_group.persons_in_group.push(person5);

        seven_group
            .persons_in_group
            .swap(seven_member as usize, (group_size - 1) as usize);

        seven_group.persons_in_group.pop();
        seven_group.persons_in_group.push(person8);

        eight_group
            .persons_in_group
            .swap(eight_member as usize, (group_size - 1) as usize);

        eight_group.persons_in_group.pop();
        eight_group.persons_in_group.push(person7);

        timeslots.swap(first_group_number as usize, (group_count - 1) as usize);
        timeslots.pop();
        timeslots.push(first_group);
        timeslots.swap(first_group_number as usize, (group_count - 1) as usize);

        timeslots.swap(second_group_number, (group_count - 1) as usize);
        timeslots.pop();
        timeslots.push(second_group);
        timeslots.swap(second_group_number, (group_count - 1) as usize);

        timeslots.swap(third_group_number, (group_count - 1) as usize);
        timeslots.pop();
        timeslots.push(third_group);
        timeslots.swap(third_group_number, (group_count - 1) as usize);

        timeslots.swap(fourth_group_number, (group_count - 1) as usize);
        timeslots.pop();
        timeslots.push(fourth_group);
        timeslots.swap(fourth_group_number, (group_count - 1) as usize);

        timeslots.swap(five_group_number as usize, (group_count - 1) as usize);
        timeslots.pop();
        timeslots.push(five_group);
        timeslots.swap(five_group_number as usize, (group_count - 1) as usize);

        timeslots.swap(six_group_number, (group_count - 1) as usize);
        timeslots.pop();
        timeslots.push(six_group);
        timeslots.swap(six_group_number, (group_count - 1) as usize);

        timeslots.swap(seven_group_number, (group_count - 1) as usize);
        timeslots.pop();
        timeslots.push(seven_group);
        timeslots.swap(seven_group_number, (group_count - 1) as usize);

        timeslots.swap(eight_group_number, (group_count - 1) as usize);
        timeslots.pop();
        timeslots.push(eight_group);
        timeslots.swap(eight_group_number, (group_count - 1) as usize);

        output.swap(counter, (time_shifts - 1) as usize);
        output.pop();
        output.push(timeslots);
        output.swap(counter, (time_shifts - 1) as usize);
        counter += 1;
    }
    output
}
