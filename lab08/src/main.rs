use std::{fs,io, collections::HashMap};
fn main()
{
    match count_data_apparition()
    {
        Ok(word_apparition)=>
        {
            let mut sorted_map :Vec<(String,u32)> =word_apparition.into_iter().collect();
            sorted_map.sort_by(|x,y| y.1.cmp(&x.1));
            let mut max_apparition = 0;
            for object in &sorted_map
            {
                if max_apparition<object.0.len()
                {
                    max_apparition=object.0.len();
                }
            }
            for object in sorted_map
            {
                let mut word = object.0.clone();
                if max_apparition != object.0.len()
                {
                    let spaces = " ".repeat(max_apparition - object.0.len());  
                    word.push_str(spaces.as_str());
                }
                println!("{} => {}",word,object.1);
            }

        },
        Err(error)=>println!("Error: {}",error),
    }
}

fn count_data_apparition()->Result<HashMap<String,u32>,io::Error>
{
    let text = fs::read_to_string("src/random.txt")?;

    let mut word_apparition: HashMap<String, u32> = HashMap::new();

    let mut current_word = String::from("");

    for character in text.chars() 
    {
        if character.is_ascii_whitespace() || character.is_ascii_punctuation()
        {
            if !current_word.is_empty()
            {
                match word_apparition.entry(current_word.clone())
                {
                    std::collections::hash_map::Entry::Occupied(mut occupied)=>
                    {
                        let value = occupied.get_mut();
                        *value+=1;
                    }
                    std::collections::hash_map::Entry::Vacant(vacant)=>
                    {
                        vacant.insert(1);
                    }
                }
                current_word.clear();
            }
        }
        else {
            current_word.push(character.to_ascii_lowercase());
        }
    }

    Ok(word_apparition)
}