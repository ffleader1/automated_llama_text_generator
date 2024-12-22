
const TEMPLATE: &str = "Your mission is to produce a yaml format answer that is in the below yaml format \
that rates a llm prompt on 4 category Experience, Knowledge, Ambiguity and Complexity. Each category \
has 2 attributes: Note and Score. Note is an array of string, contain points what you want to talk \
about the category. Rating conclude what should the category be ranked as. It can be one of: Easy, \
Easy - Medium, Medium, Medium - Hard, Hard and Very Hard.\
There should be the Overall rating also, which strictly falls into Easy, Medium and Hard.\
Sample response (do not include the backtick in the answer.
```
# Experience
  Note:
    - Need experience about optimizing Rust calculation
  Rating: Medium
Knowledge
  Note:
    - Both Math knowledge and Rust knowledge is required
  Rating: Medium
Ambiguity
  Note:
    - Prompt is clear on the point, overall goal, and even included what to avoid
    - Prompt did not go into details what step to take
    - Provided code is long
  Rating: Medium
Complexity
  Note:
    - Have to use non standard library or some high level optimization
  Rating: Medium - Hard
Overall: Medium
```
You will be provided with the prompt itself, and, optionally, the previous turn answer that lead to
the prompt.
---
The prompt:
{CURRENT_PROMPT}

The previous turn answer:
{PREVIOUS_TURN_ANSWER}
---
Some overall guide on what to decide on the prompt:

Easy:

Experience level: undergraduate level
Knowledge required: limited domain/algorithmics knowledge or implementation context (architecture, libraries, pre-existing code)
Ambiguity of prompt: little ambiguity in the question (in case of underspecification, good default behaviors are easy to come up with or not important), limited complexity of specifications (in #instructions)
Complexity of solution: solution is easy to explain (e.g., code doesn’t need comments to be understood) and to test for/debug (limited corner cases)


Medium:

Experience level: masters level
Knowledge required: may require knowledge of standard algorithms and data structures to get an optimal solution, knowledge of common libraries and concepts or additional code context.
Ambiguity of prompt: medium ambiguity in the prompt (e.g., needs to come up with reasonable ad-hoc data representation or class structure without explicit guidance), multiple requirements should be satisfied or multiple bugs should be found
Complexity of solution: involves corner cases that should be dealt with separately; explanation of the solution requires some abstraction or decomposition of the problem into a few subproblems


Hard:

Experience level: domain expert question
Knowledge required: require expert domain knowledge, or information on the specific application or deployment scenario, including substantial specific API/code context
Ambiguity of prompt: finding good solutions need non-trivial design decisions regarding data structures, algorithms or code architecture/design patterns
Complexity of solution: finding a solution requires solving several non-trivial subproblems or finding non-trivial bugs; problem involves tricky corner cases, explaining the solution to a non-expert requires adding context

Some example just for you
";

pub fn generate_chat_gpt_prompt(current_prompt: String, previous_turn: String) -> String {
    let prev_turn_str = if previous_turn.is_empty() {
        "(none)".to_string()
    }else{
        previous_turn
    };
    TEMPLATE
        .replace("{CURRENT_PROMPT}", &current_prompt)
        .replace("{PREVIOUS_TURN_ANSWER}", &prev_turn_str)
}