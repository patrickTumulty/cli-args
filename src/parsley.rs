use std::collections::HashMap;

use crate::{Flag, ResContainer, Arg};

pub struct Parsley {
    res_container: ResContainer,
    cli_flags: Vec<Flag>,
    cli_args: Vec<Arg>,
}

impl Parsley {
    pub fn parse_input_args(&mut self, input_args: &mut Vec<String>) {
        let app_name = input_args.remove(0);

        let mut arg_map: HashMap<String, usize> = HashMap::new();
        let mut flag_map: HashMap<String, usize> = HashMap::new();

        self.populate_arg_map(&mut arg_map, &self.cli_args);

        self.populate_flag_map(&mut flag_map, &self.cli_flags);

        for mut i in 0..input_args.len() {
            let input_arg = &input_args[i];

            if let Some(flag_index) = flag_map.get(input_arg) {
                let flag = &self.cli_flags[flag_index.clone()];
                flag.run_action(&mut self.res_container, input_args.remove(i));
                i = 0;
            }
        }

    }

    fn populate_arg_map(&self, arg_map: &mut HashMap<String, usize>,
                        args: &Vec<Arg>) 
    {
        for i in 0..args.len() {
            let arg = &args[i];
            arg_map.insert(arg.name().clone(), i);
            if let Some(alt_arg_name) = arg.alt_name() {
                arg_map.insert(alt_arg_name.clone(), i);
            }
        }
    }

    fn populate_flag_map(&self, flag_map: &mut HashMap<String, usize>,
                         flags: &Vec<Flag>) 
    {
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
    arg_actions: Vec<Arg>,
    arg_flags: Vec<Flag>,
}

impl ParsleyBuilder {
    pub fn new() -> ParsleyBuilder {
        ParsleyBuilder {
            res_container: ResContainer::new(),
            arg_flags: Vec::new(),
            arg_actions: Vec::new(),
        }
    }

    pub fn add_resource<T: 'static>(mut self, resource: T) -> ParsleyBuilder {
        self.res_container.add::<T>(resource);
        self
    }

    pub fn add_flag(mut self, flag: Flag) -> ParsleyBuilder {
        self.arg_flags.push(flag);
        self
    }

    pub fn add_action(mut self, action: Arg) -> ParsleyBuilder {
        self.arg_actions.push(action);
        self
    }

    pub fn build(mut self) -> Parsley {
        return Parsley {
            res_container: self.res_container,
            cli_args: self.arg_actions,
            cli_flags: self.arg_flags,
        };
    }
}
