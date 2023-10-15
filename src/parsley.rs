use crate::{Arg, Flag, ResContainer};
use std::collections::{HashMap, VecDeque};
use std::env;

pub struct Parsley {
    res_container: ResContainer,
    app_name: String,
    cli_flags: Vec<Flag>,
    cli_args: Vec<Arg>,
}

impl Parsley {
    pub fn parse_input_args(&mut self) {
        let mut input_args: Vec<String> = env::args().collect();

        self.set_app_name(&mut input_args);

        let mut arg_map: HashMap<String, usize> = HashMap::new();
        let mut flag_map: HashMap<String, usize> = HashMap::new();

        self.populate_flag_map(&mut flag_map, &self.cli_flags);
        self.populate_arg_map(&mut arg_map, &self.cli_args);

        self.process_flags(flag_map, &mut input_args);
        self.process_args(arg_map, &mut input_args);
    }

    fn process_flags(&mut self, flag_map: HashMap<String, usize>, input_args: &mut Vec<String>) {
        let mut i: usize = 0;
        loop {
            if i >= input_args.len() {
                break;
            }

            let input_arg = &input_args[i];

            if let Some(flag_index) = flag_map.get(input_arg) {
                let flag = &self.cli_flags[flag_index.clone()];
                flag.run_action(&mut self.res_container, input_args.remove(i));
                i = 0;
                continue;
            }

            i += 1;
        }
    }

    fn process_args(&mut self, arg_map: HashMap<String, usize>, input_args: &mut Vec<String>) {
        let mut i: usize = 0;
        loop {
            if i >= input_args.len() {
                break;
            }

            let input_arg = &input_args[i];

            if let Some(flag_index) = arg_map.get(input_arg) {
                let arg = &self.cli_args[flag_index.clone()];
                let mut inputs: Vec<String> = Vec::new();
                input_args.remove(i); // Discard arg name
                for _ in 0..*arg.expected_arg_values_count() {
                    if i < input_args.len() {
                        inputs.push(input_args.remove(i));
                    }
                }
                arg.run_action(&mut self.res_container, inputs);
                i = 0;
                continue;
            }

            i += 1;
        }
    }

    fn set_app_name(&mut self, input_args: &mut Vec<String>) {
        let mut app_name = input_args.remove(0);
        let last_separator = app_name.rfind('/');
        if last_separator.is_some() {
            app_name = app_name.split_off(last_separator.unwrap() + 1);
        }
        self.app_name = app_name;
    }

    fn populate_arg_map(&self, arg_map: &mut HashMap<String, usize>, args: &Vec<Arg>) {
        for i in 0..args.len() {
            let arg = &args[i];
            arg_map.insert(arg.name().clone(), i);
            if let Some(alt_arg_name) = arg.alt_name() {
                arg_map.insert(alt_arg_name.clone(), i);
            }
        }
    }

    fn populate_flag_map(&self, flag_map: &mut HashMap<String, usize>, flags: &Vec<Flag>) {
        for i in 0..flags.len() {
            let flag = &flags[i];
            flag_map.insert(flag.name().clone(), i);
            if let Some(alt_arg_name) = flag.alt_name() {
                flag_map.insert(alt_arg_name.clone(), i);
            }
        }
    }
}

pub struct ParsleyBuilder {
    res_container: ResContainer,
    cli_args: Vec<Arg>,
    cli_flags: Vec<Flag>,
}

impl ParsleyBuilder {
    pub fn new() -> ParsleyBuilder {
        ParsleyBuilder {
            res_container: ResContainer::new(),
            cli_flags: Vec::new(),
            cli_args: Vec::new(),
        }
    }

    pub fn add_resource<T: 'static>(mut self, resource: T) -> ParsleyBuilder {
        self.res_container.add::<T>(resource);
        self
    }

    pub fn add_flag(mut self, flag: Flag) -> ParsleyBuilder {
        self.cli_flags.push(flag);
        self
    }

    pub fn add_arg(mut self, arg: Arg) -> ParsleyBuilder {
        self.cli_args.push(arg);
        self
    }

    pub fn build(self) -> Parsley {
        return Parsley {
            res_container: self.res_container,
            app_name: "".to_string(),
            cli_args: self.cli_args,
            cli_flags: self.cli_flags,
        };
    }
}
