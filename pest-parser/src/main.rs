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
use pest_parser::parse_sapl_file;
use pest_parser::Rule;
use pest_parser::SaplParser;

fn main() {
    let parse_result = SaplParser::parse(
        Rule::sapl_document,
        "policy \"policy 1\" deny transform resource.content |- filter.blacken",
    )
    .unwrap();
    let tokens = parse_result.tokens();

    for token in tokens {
        println!("{:?}", token);
    }

    let policy = parse_sapl_file("policy \"policy 1\" deny");
    println!("{:?}", policy);

    let policy_set = parse_sapl_file(
        "set \"classified documents\" first-applicable policy \"Clearance (1/3)\" permit",
    );
    println!("{:?}", policy_set);

    let import = parse_sapl_file("import filter as filter policy \"policy\" permit");
    println!("{:?}", import);

    let schema = parse_sapl_file("subject schema aSubjectSchema policy \"policy schema\" deny");
    println!("{:?}", schema);

    let transform =
        parse_sapl_file("policy \"test\" permit transform resource.content |- filter.blacken");
    println!("{:?}", transform);

    let advice = parse_sapl_file("policy \"policy 1\" deny advice \"logging:inform_admin\"");
    println!("{:?}", advice);

    let obligation = parse_sapl_file("policy \"test\" permit obligation \"logging:log_access\"");
    println!("{:?}", obligation);

    let where_statement = parse_sapl_file("policy \"test_policy\" permit where var variable = \"anAttribute\"; subject.attribute == variable; var foo = true schema {\"type\": \"boolean\"}");
    println!("{:?}", where_statement);
}
