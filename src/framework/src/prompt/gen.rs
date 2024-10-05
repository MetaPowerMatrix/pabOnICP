use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub fn generate_prompt<T: AsRef<str> + ToString, P: AsRef<Path>>(
    curr_input: Vec<T>,
    prompt_lib_file: P,
) -> io::Result<String> {
    // Read the prompt file content
    let mut file = File::open(prompt_lib_file)?;
    let mut prompt = String::new();
    file.read_to_string(&mut prompt)?;

    // Replace placeholder with actual inputs
    for (count, input) in curr_input.iter().enumerate() {
        let placeholder = format!("!<INPUT {}>!", count);
        prompt = prompt.replace(&placeholder, input.as_ref());
    }

    // Split at comment block marker and return the latter part if exists
    let prompt_parts: Vec<&str> = prompt.split("<commentblockmarker>###</commentblockmarker>").collect();
    let final_prompt = if prompt_parts.len() > 1 { prompt_parts[1] } else { prompt_parts[0] };

    Ok(final_prompt.trim().to_string())
}
