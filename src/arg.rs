use crate::resource::ResContainer;


pub struct Arg {
    name: String,
    alt_name: Option<String>,
    description: String,
    expected_arg_values_count: u32,
    action_args: Vec<String>,
    action: fn(&mut ResContainer, Vec<String>)
}

impl Arg {
    pub fn new(name: &str,
               short_name: Option<&str>,
               description: &str,
               expected_arg_values_count: u32,
               action: fn(&mut ResContainer, Vec<String>)) -> Self
    {
        return Arg {
            name: name.to_string(),
            alt_name: if short_name.is_some() { Some(short_name.unwrap().to_string()) } else { None },
            description: description.to_string(),
            expected_arg_values_count,
            action_args: Vec::new(),
            action
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn alt_name(&self) -> &Option<String> {
        &self.alt_name
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn expected_arg_values_count(&self) -> &u32 {
        &self.expected_arg_values_count
    }

    pub fn arg_values(&mut self) -> &mut Vec<String> {
        &mut self.action_args
    }

    pub fn run_action(&self, res_conteiner: &mut ResContainer, value: Vec<String>) {
        (self.action)(res_conteiner, value);
    }
}


