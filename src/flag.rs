use crate::ResContainer;

pub struct Flag { 
    name: String, 
    alt_name: Option<String>,
    description: String, 
    action: fn(&mut ResContainer, String)
}

impl Flag {
    pub fn new(name: &str,
               alt_name: Option<&str>,
               description: &str,
               action: fn(&mut ResContainer, String)) -> Flag 
    {
        return Flag {
            name: name.to_string(),
            alt_name: if alt_name.is_some() { Some(alt_name.unwrap().to_string()) } else { None },
            description: description.to_string(), 
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

    pub fn run_action(&self, res_conteiner: &mut ResContainer, value: String) {
        (self.action)(res_conteiner, value);
    }
}

