/*
    Copyright 2025 Stefan Weng

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

use pest_parser::parse_sapl_file;

fn main() {
    {
        let policy_new = parse_sapl_file("policy \"policy 1\" permit");
        println!("{:#?}", policy_new);
        println!();
    }
    {
        let policy_set_new = parse_sapl_file("set \"classified documents\" first-applicable policy \"Clearance (1/3)\" permit policy \"test policy\" deny");
        println!("{:#?}", policy_set_new);
        println!();
    }
    {
        let policy_set_new = parse_sapl_file("import filter.blacken import filter as filter import filter as filter import filter.* import sapl.pip.http.* set \"classified documents\" first-applicable policy \"Clearance (1/3)\" permit policy \"test policy\" permit a == b && c == d | e > f");
        println!("{:#?}", policy_set_new);
    }
    {
        let schema = parse_sapl_file("subject schema aSubjectSchema policy \"policy schema\" deny");
        println!("{:#?}", schema);
        println!();
    }
    {
        let schema =
            parse_sapl_file("subject schema aSubjectSchema == true policy \"policy schema\" deny");
        println!("{:#?}", schema);
        println!();
    }
    {
        let schema = parse_sapl_file(
            "subject schema { \"title\": \"Person\" } policy \"policy schema\" permit",
        );
        println!("{:#?}", schema);
        println!();
    }
    {
        let schema = parse_sapl_file("subject schema { \"firstName\": {             \"type\": \"string\", \"description\": \"The person's first name.\" }} policy \"test pair\" deny",
        );
        println!("{:#?}", schema);
        println!();
    }
    {
        let schema = parse_sapl_file(
            "subject schema { \"title\": \"Person\", \"tile2\": \"Next\" } policy \"policy schema\" permit",
        );
        println!("{:#?}", schema);
        println!();
    }
    {
        let schema = parse_sapl_file(
            "subject enforced schema 
    {
    \"$id\": \"https://example.com/person.schema.json\",
    \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",
    \"title\": \"Person\",
    \"type\": \"object\",
    \"properties\": {
        \"firstName\": {
            \"type\": \"string\",
            \"description\": \"The person's first name.\"
        },
        \"lastName\": {
            \"type\": \"string\",
            \"description\": \"The person's last name.\"
        },
        \"age\": {
            \"description\": \"Age in years which must be equal to or greater than zero.\",
            \"type\": \"integer\",
            \"minimum\": 0
        }
    }
    } policy \"policy schema\" deny",
        );
        println!("{:#?}", schema);
        println!();
    }
    {
        let transformation =
            parse_sapl_file("policy \"test\" permit transform resource.content |- filter.blacken");
        println!("{:#?}", transformation);
        println!();
    }
    {
        let transformation =
            parse_sapl_file("policy \"test\" permit transform { \"id\": resource.id, \"name\": resource.name,\"phoneNumber\": resource.phoneNumber}");
        println!("{:#?}", transformation);
        println!();
    }
    {
        let policy_set = parse_sapl_file(
            "set \"classified documents\" first-applicable policy \"Clearance (1/3)\" permit",
        );
        println!("{:?}", policy_set);
        println!();
    }
    // let import = parse_sapl_file("import filter as filter policy \"policy\" permit");
    // println!("{:?}", import);
    //
    // let schema = parse_sapl_file("subject schema aSubjectSchema policy \"policy schema\" deny");
    // println!("{:?}", schema);
    //
    // let transform =
    //     parse_sapl_file("policy \"test\" permit transform resource.content |- filter.blacken");
    // println!("{:?}", transform);
    {
        let advice = parse_sapl_file("policy \"policy 1\" deny advice \"logging:inform_admin\"");
        println!("{:#?}", advice);
        println!();
    }
    {
        let advice = parse_sapl_file("policy \"policy 1\" deny advice { \"type\": \"logAccess\", \"message\": (\"Administrator \" + subject.name + \" has manipulated patient: \" + action.http.requestedURI) }");
        println!("{:#?}", advice);
        println!();
    }
    {
        let obligation =
            parse_sapl_file("policy \"test\" permit obligation \"logging:log_access\"");
        println!("{:#?}", obligation);
        println!();
    }
    {
        let obligation =
            parse_sapl_file("policy \"test\" permit obligation { \"type\": \"sendEmail\", \"recipient\": patient.attendingDoctor, \"subject\": \"Data of your patient \" + (patient.name) + \" was changed.\", \"message\": \"Doctor \" + subject.name + \" changed the data.\" }");
        println!("{:#?}", obligation);
        println!();
    }
    {
        let where_statement = parse_sapl_file("policy \"test_policy\" permit where var variable = \"anAttribute\"; subject.attribute == variable; var foo = true schema {\"type\": \"boolean\"}");
        println!("{:#?}", where_statement);
        println!();
    }
    {
        let policy_with_unvalid_target_expr =
            parse_sapl_file("policy \"policy 1\" permit false && true || false");
        if let Ok(sapl_doc) = policy_with_unvalid_target_expr {
            match sapl_doc.validate() {
                Ok(()) => println!("Sapl document {} successfully validated.", sapl_doc.name()),
                Err(msg) => println!("{}", msg),
            }
            println!();
        };
    }
    {
        let policy_with_unvalid_target_expr =
            parse_sapl_file("policy \"policy 2\" permit false & true | false");
        if let Ok(sapl_doc) = policy_with_unvalid_target_expr {
            match sapl_doc.validate() {
                Ok(()) => println!("Sapl document {} successfully validated.", sapl_doc.name()),
                Err(msg) => println!("{}", msg),
            }
            println!();
        };
    }
    {
        let policy_with_unvalid_target_expr =
            parse_sapl_file("policy \"policy 3\" permit subject.name == resource.id.<patient.patientRecord>.attendingDoctor");
        if let Ok(sapl_doc) = policy_with_unvalid_target_expr {
            match sapl_doc.validate() {
                Ok(()) => println!("Sapl document {} successfully validated.", sapl_doc.name()),
                Err(msg) => println!("{}", msg),
            }
            println!();
        };
    }
    {
        let schema =
            parse_sapl_file("subject schema aSubjectSchema == false policy \"policy schema\" deny");
        println!("{:#?}", schema);
        println!();
    }
}
