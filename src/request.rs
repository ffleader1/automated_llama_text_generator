
use anyhow::Result;
use crate::prompt;
use crate::raw_example;

pub fn gen_request_content(current_prompt: String, previous_turn: String, preference_difficulty: usize,
                           preference_length: usize) -> Result<String> {
    if current_prompt.is_empty() {
        return Err(anyhow::anyhow!("Prompt cannot be empty."));
    }
    let mut chat_gpt_prompt = prompt::generate_chat_gpt_prompt(current_prompt,previous_turn);
    chat_gpt_prompt.push_str(&raw_example::generate_sample());

    if preference_difficulty > 0 {

        chat_gpt_prompt.push_str(&format!("\nI do have a preference for the overall rating of {}\n\
        So you are welcome to weak your words to get that overall rating. \
        That is the overall rating, not the component rating, so feel free to wiggle the component rating
        if possible to make it sounds fair.
        Of course, being reasonable is important, so if you tried hard but cannot, it's fine.\
        ",  match preference_difficulty {
            1 => "Easy",
            2 => "Medium",
            _ => "Hard",
        }));
    }

    chat_gpt_prompt.push_str("Avoid if possible putting all 4 sub rating to be the same thing.\
    That does not sound like a subjective judgement\n");

    chat_gpt_prompt.push_str(match preference_length {
        0 => "\nFinally. I would like a simple answer, so I strongly prefer no more than 2 points \
        per category, as the absolute max should be 3. Also, if you can, please put 1\n",
        2 => "\nFinally. I would like a long answer, so feel free to add  as many point as possible\
        to describe your selection\n",
        _ => "\nFinally. I would like a simple answer, so I absolutely \
        want no more than 5 points per category, and most category should be between 2-3 points\n",
    });
    Ok(chat_gpt_prompt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_data() {
        match gen_request_content("gen hello world".to_string(), "".to_string(),0,0)  {
            Ok(r) => {
                println!("{}", r);
            }
            Err(e) => {
                println!("{}", format!("{:?}", e));
            }
        }
    }
}