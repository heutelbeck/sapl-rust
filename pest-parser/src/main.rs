/*
    Copyright 2024 Stefan Weng

    Licensed under the Apache License, Version 2.0 (the "License"); you may not
    use this file except in compliance with the License. You may obtain a copy
    of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
    WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
    License for the specific language governing permissions and limitations
    under the License.
*/

use pest::Parser;
use pest_parser::Decision;
use pest_parser::Rule;
use pest_parser::{parse, SaplParser};

fn main() {
    let parse_result = SaplParser::parse(Rule::policy, "policy \"policy 1\" deny").unwrap();
    let tokens = parse_result.tokens();

    for token in tokens {
        println!("{:?}", token);
    }

    println!("Now SaplDocument Rule:");
    let decision = parse("policy \"policy 1\" deny");
    assert_eq!(decision.get_decision(), Decision::Deny);
}
