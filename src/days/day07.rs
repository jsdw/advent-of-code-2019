use crate::error::Error;
use crate::support::intcode::{ Intcode, Outcome, parse_intcode_ops };
use itertools::Itertools;

pub fn both_parts(input: &str) -> Result<(), Error> {
    let intcode = Intcode::new(parse_intcode_ops(input)?);

    // What combination of inputs in the range [0,4] produces the
    // largest final output?
    let mut star1_output = std::i64::MIN;
    for ns in (0..=4).permutations(5) {
        if let &[a,b,c,d,e] = &*ns {
            star1_output = run_amplifiers_repeatedly_with_input(vec![a,b,c,d,e], &intcode)?.max(star1_output);
        }
    }
    println!("Star 1: {}", star1_output);

    // What combination of inputs in the range [5,9] produces the
    // largest final output?
    let mut star2_output = std::i64::MIN;
    for ns in (5..=9).permutations(5) {
        if let &[a,b,c,d,e] = &*ns {
            star2_output = run_amplifiers_repeatedly_with_input(vec![a,b,c,d,e], &intcode)?.max(star2_output);
        }
    }
    println!("Star 2: {}", star2_output);

    Ok(())
}

/// This seems to work for parts 1 and 2; keep running amplifiers until they halt and return
/// the last value that we get back before halting occurs.
fn run_amplifiers_repeatedly_with_input(inputs: Vec<i64>, intcode: &Intcode) -> Result<i64,Error> {

    // Each of these is a function that takes amplifier input in and returns
    // an output, or nothing if the intcode machine has halted.
    let mut intcode_funcs: Vec<_> = inputs
        .into_iter()
        .map(|first_input| intcode_fn(intcode, first_input))
        .collect();

    // Run these repeatedly until they halt
    let mut next_input = 0;
    for idx in (0..intcode_funcs.len()).cycle() {
        let f = &mut intcode_funcs[idx];
        if let Some(n) = f(next_input)? {
            next_input = n;
        } else {
            break
        }
    }

    // Return the last input we got out before everything halts
    // (This seems to suffice)
    Ok(next_input)
}

/// Turn an intcode tempalte into a function which takes inputs and
/// returns outputs, progressing the intcode machine each time it runs.
fn intcode_fn(intcode: &Intcode, first_input: i64) -> impl FnMut(i64) -> Result<Option<i64>,Error> {
    let mut intcode = intcode.clone();
    let mut has_used_first_input = false;
    move |next_input: i64| -> Result<Option<i64>,Error> {
        while let Some(outcome) = intcode.step()? {
            match outcome {
                Outcome::NeedsInput(provider) => {
                    if !has_used_first_input {
                        provider.provide(first_input);
                        has_used_first_input = true;
                    } else {
                        provider.provide(next_input);
                    }
                },
                Outcome::Output(value) => {
                    return Ok(Some(value));
                },
                Outcome::StepComplete => {
                    /* keep calm and carry on */
                }
            }
        }
        // Halted; no more value to give.
        Ok(None)
    }
}