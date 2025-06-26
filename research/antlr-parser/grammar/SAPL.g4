/*
    Copyright 2025 Dominic Heutelbeck (dominic@heutelbeck.com)

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

grammar SAPL;

saplDocument: importStatement* schema* policyElement EOF;

importStatement: 'import' ( wildcardImport | libraryImport | functionImport );

wildcardImport: saplID ('.' saplID)* '.' '*' ;

libraryImport: saplID ('.' saplID)* 'as' saplID ;

functionImport: saplID ('.' saplID)* '.' saplID ;

schema: reservedID ( 'enforced' )? 'schema' expression ;

policyElement: policySet | policy ;

policySet: 'set' STRING combiningAlgorithm ('for' targetExpression)?
            ( valueDefinition ';' )* policy+ ;

combiningAlgorithm: 'deny-overrides'
                  | 'permit-overrides'
                  | 'first-applicable'
                  | 'only-one-applicable'
                  | 'deny-unless-permit'
                  | 'permit-unless-deny' ;

policy: 'policy' STRING entitlement ( targetExpression )?
        ( 'where' statement ( statement )* )?
        ( 'obligation' expression )*
        ( 'advice' expression )*
        ( 'transform' expression )* ;

entitlement: 'permit' | 'deny' ;

targetExpression: expression ;

statement: (valueDefinition | expression) ';' ;

valueDefinition: 'var' ID '=' expression ( 'schema' expression ( ',' expression )* )? ;

expression: lazyOr ;

lazyOr: lazyAnd ( '||' lazyAnd )* ;

lazyAnd: eagerOr ( '&&' eagerOr )* ;

eagerOr: exclusiveOr ( '|' exclusiveOr )* ;

exclusiveOr: eagerAnd ( '^' eagerAnd )* ;

eagerAnd: equality ( '&' equality )* ;

equality: comparison ( ( '==' | '!=' | '=~' ) comparison )? ;

comparison: addition ( ( '<' | '<=' | '>' | '>=' | 'in' ) addition )? ;

addition: multiplication ( ( '+' | '-' ) multiplication )* ;

multiplication: unaryExpression ( ( '*' | '/' | '%' ) unaryExpression )* ;

unaryExpression: ( '!' | '-' | '+' ) unaryExpression | basicExpression ;

basicExpression: basic ( FILTER filterComponent | SUBTEMPLATE basicExpression )? ;

basic: '(' expression ')' step* 
     | value step* 
     | saplID ('.' saplID)* arguments step*
     | '<' saplID ('.' saplID)* arguments? '>' step*
     | '|<' saplID ('.' saplID)* arguments? '>' step*
     | saplID step*
     | '@' step* ;

arguments: '(' ( expression ( ',' expression )* )? ')' ;

step: '.' ( saplID | STRING | '*' | '<' saplID ('.' saplID)* arguments? '>' | '|<' saplID ('.' saplID)* arguments? '>' )
     | '[' subscript ']' 
     | '..' ( ( saplID | '[' STRING ']' ) | ( '*' | '[' '*' ']' ) | '[' signedNumber ']' ) ;

subscript: STRING | '*' | signedNumber | signedNumber? ':' signedNumber? ( ':' signedNumber )? | '(' expression ')' | '?' '(' expression ')' | signedNumber ( ',' signedNumber )* | STRING ( ',' STRING )* ;

value: object | array | numberLiteral | stringLiteral | booleanLiteral | nullLiteral | undefinedLiteral ;

object: '{' ( pair ( ',' pair )* )? '}' ;

pair: STRING ':' expression ;

array: '[' ( expression ( ',' expression )* )? ']' ;

booleanLiteral: 'true' | 'false' ;

nullLiteral: 'null' ;

undefinedLiteral: 'undefined' ;

stringLiteral: STRING ;

numberLiteral: JSONNUMBER ;

filterComponent: 'each'? saplID ('.' saplID)* arguments?
               | '{' filterStatement ( ',' filterStatement )* '}' ;

filterStatement: 'each'? '@'? ':' saplID ('.' saplID)* arguments? ;

signedNumber: '-'? JSONNUMBER ;

saplID: ID | reservedID ;

reservedID: 'subject' | 'action' | 'resource' | 'environment' ;

ID: '^'? ('a'..'z'|'A'..'Z'|'_'|'$') ('a'..'z'|'A'..'Z'|'_'|'$'|'0'..'9')* ;

FILTER: '|-' ;

SUBTEMPLATE: '::' ;

JSONNUMBER: ('0' | ('1'..'9') DIGIT*) ('.' DIGIT+)? (('E' | 'e') ('+' | '-')? DIGIT+)? ;

fragment DIGIT: ('0'..'9') ;

WS: [ \t\r\n]+ -> skip ;

ML_COMMENT: '/*' .*? '*/' -> skip ;

SL_COMMENT: '//' ~[\r\n]* -> skip ;

STRING: '"' ( '\\' . | ~('\\'|'"') )* '"' ;
