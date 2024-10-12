#![allow(nonstandard_style)]
// Generated from grammar/SAPL.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::saplparser::*;

pub trait SAPLListener<'input> : ParseTreeListener<'input,SAPLParserContextType>{
/**
 * Enter a parse tree produced by {@link SAPLParser#saplDocument}.
 * @param ctx the parse tree
 */
fn enter_saplDocument(&mut self, _ctx: &SaplDocumentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#saplDocument}.
 * @param ctx the parse tree
 */
fn exit_saplDocument(&mut self, _ctx: &SaplDocumentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#importStatement}.
 * @param ctx the parse tree
 */
fn enter_importStatement(&mut self, _ctx: &ImportStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#importStatement}.
 * @param ctx the parse tree
 */
fn exit_importStatement(&mut self, _ctx: &ImportStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#wildcardImport}.
 * @param ctx the parse tree
 */
fn enter_wildcardImport(&mut self, _ctx: &WildcardImportContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#wildcardImport}.
 * @param ctx the parse tree
 */
fn exit_wildcardImport(&mut self, _ctx: &WildcardImportContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#libraryImport}.
 * @param ctx the parse tree
 */
fn enter_libraryImport(&mut self, _ctx: &LibraryImportContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#libraryImport}.
 * @param ctx the parse tree
 */
fn exit_libraryImport(&mut self, _ctx: &LibraryImportContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#functionImport}.
 * @param ctx the parse tree
 */
fn enter_functionImport(&mut self, _ctx: &FunctionImportContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#functionImport}.
 * @param ctx the parse tree
 */
fn exit_functionImport(&mut self, _ctx: &FunctionImportContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#schema}.
 * @param ctx the parse tree
 */
fn enter_schema(&mut self, _ctx: &SchemaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#schema}.
 * @param ctx the parse tree
 */
fn exit_schema(&mut self, _ctx: &SchemaContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#policyElement}.
 * @param ctx the parse tree
 */
fn enter_policyElement(&mut self, _ctx: &PolicyElementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#policyElement}.
 * @param ctx the parse tree
 */
fn exit_policyElement(&mut self, _ctx: &PolicyElementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#policySet}.
 * @param ctx the parse tree
 */
fn enter_policySet(&mut self, _ctx: &PolicySetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#policySet}.
 * @param ctx the parse tree
 */
fn exit_policySet(&mut self, _ctx: &PolicySetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#combiningAlgorithm}.
 * @param ctx the parse tree
 */
fn enter_combiningAlgorithm(&mut self, _ctx: &CombiningAlgorithmContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#combiningAlgorithm}.
 * @param ctx the parse tree
 */
fn exit_combiningAlgorithm(&mut self, _ctx: &CombiningAlgorithmContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#policy}.
 * @param ctx the parse tree
 */
fn enter_policy(&mut self, _ctx: &PolicyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#policy}.
 * @param ctx the parse tree
 */
fn exit_policy(&mut self, _ctx: &PolicyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#entitlement}.
 * @param ctx the parse tree
 */
fn enter_entitlement(&mut self, _ctx: &EntitlementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#entitlement}.
 * @param ctx the parse tree
 */
fn exit_entitlement(&mut self, _ctx: &EntitlementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#targetExpression}.
 * @param ctx the parse tree
 */
fn enter_targetExpression(&mut self, _ctx: &TargetExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#targetExpression}.
 * @param ctx the parse tree
 */
fn exit_targetExpression(&mut self, _ctx: &TargetExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#statement}.
 * @param ctx the parse tree
 */
fn enter_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#statement}.
 * @param ctx the parse tree
 */
fn exit_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#valueDefinition}.
 * @param ctx the parse tree
 */
fn enter_valueDefinition(&mut self, _ctx: &ValueDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#valueDefinition}.
 * @param ctx the parse tree
 */
fn exit_valueDefinition(&mut self, _ctx: &ValueDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#lazyOr}.
 * @param ctx the parse tree
 */
fn enter_lazyOr(&mut self, _ctx: &LazyOrContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#lazyOr}.
 * @param ctx the parse tree
 */
fn exit_lazyOr(&mut self, _ctx: &LazyOrContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#lazyAnd}.
 * @param ctx the parse tree
 */
fn enter_lazyAnd(&mut self, _ctx: &LazyAndContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#lazyAnd}.
 * @param ctx the parse tree
 */
fn exit_lazyAnd(&mut self, _ctx: &LazyAndContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#eagerOr}.
 * @param ctx the parse tree
 */
fn enter_eagerOr(&mut self, _ctx: &EagerOrContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#eagerOr}.
 * @param ctx the parse tree
 */
fn exit_eagerOr(&mut self, _ctx: &EagerOrContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#exclusiveOr}.
 * @param ctx the parse tree
 */
fn enter_exclusiveOr(&mut self, _ctx: &ExclusiveOrContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#exclusiveOr}.
 * @param ctx the parse tree
 */
fn exit_exclusiveOr(&mut self, _ctx: &ExclusiveOrContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#eagerAnd}.
 * @param ctx the parse tree
 */
fn enter_eagerAnd(&mut self, _ctx: &EagerAndContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#eagerAnd}.
 * @param ctx the parse tree
 */
fn exit_eagerAnd(&mut self, _ctx: &EagerAndContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#equality}.
 * @param ctx the parse tree
 */
fn enter_equality(&mut self, _ctx: &EqualityContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#equality}.
 * @param ctx the parse tree
 */
fn exit_equality(&mut self, _ctx: &EqualityContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#comparison}.
 * @param ctx the parse tree
 */
fn enter_comparison(&mut self, _ctx: &ComparisonContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#comparison}.
 * @param ctx the parse tree
 */
fn exit_comparison(&mut self, _ctx: &ComparisonContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#addition}.
 * @param ctx the parse tree
 */
fn enter_addition(&mut self, _ctx: &AdditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#addition}.
 * @param ctx the parse tree
 */
fn exit_addition(&mut self, _ctx: &AdditionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#multiplication}.
 * @param ctx the parse tree
 */
fn enter_multiplication(&mut self, _ctx: &MultiplicationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#multiplication}.
 * @param ctx the parse tree
 */
fn exit_multiplication(&mut self, _ctx: &MultiplicationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#unaryExpression}.
 * @param ctx the parse tree
 */
fn enter_unaryExpression(&mut self, _ctx: &UnaryExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#unaryExpression}.
 * @param ctx the parse tree
 */
fn exit_unaryExpression(&mut self, _ctx: &UnaryExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#basicExpression}.
 * @param ctx the parse tree
 */
fn enter_basicExpression(&mut self, _ctx: &BasicExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#basicExpression}.
 * @param ctx the parse tree
 */
fn exit_basicExpression(&mut self, _ctx: &BasicExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#basic}.
 * @param ctx the parse tree
 */
fn enter_basic(&mut self, _ctx: &BasicContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#basic}.
 * @param ctx the parse tree
 */
fn exit_basic(&mut self, _ctx: &BasicContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#arguments}.
 * @param ctx the parse tree
 */
fn enter_arguments(&mut self, _ctx: &ArgumentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#arguments}.
 * @param ctx the parse tree
 */
fn exit_arguments(&mut self, _ctx: &ArgumentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#step}.
 * @param ctx the parse tree
 */
fn enter_step(&mut self, _ctx: &StepContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#step}.
 * @param ctx the parse tree
 */
fn exit_step(&mut self, _ctx: &StepContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#subscript}.
 * @param ctx the parse tree
 */
fn enter_subscript(&mut self, _ctx: &SubscriptContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#subscript}.
 * @param ctx the parse tree
 */
fn exit_subscript(&mut self, _ctx: &SubscriptContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#value}.
 * @param ctx the parse tree
 */
fn enter_value(&mut self, _ctx: &ValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#value}.
 * @param ctx the parse tree
 */
fn exit_value(&mut self, _ctx: &ValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#object}.
 * @param ctx the parse tree
 */
fn enter_object(&mut self, _ctx: &ObjectContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#object}.
 * @param ctx the parse tree
 */
fn exit_object(&mut self, _ctx: &ObjectContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#pair}.
 * @param ctx the parse tree
 */
fn enter_pair(&mut self, _ctx: &PairContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#pair}.
 * @param ctx the parse tree
 */
fn exit_pair(&mut self, _ctx: &PairContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#array}.
 * @param ctx the parse tree
 */
fn enter_array(&mut self, _ctx: &ArrayContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#array}.
 * @param ctx the parse tree
 */
fn exit_array(&mut self, _ctx: &ArrayContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#booleanLiteral}.
 * @param ctx the parse tree
 */
fn enter_booleanLiteral(&mut self, _ctx: &BooleanLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#booleanLiteral}.
 * @param ctx the parse tree
 */
fn exit_booleanLiteral(&mut self, _ctx: &BooleanLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#nullLiteral}.
 * @param ctx the parse tree
 */
fn enter_nullLiteral(&mut self, _ctx: &NullLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#nullLiteral}.
 * @param ctx the parse tree
 */
fn exit_nullLiteral(&mut self, _ctx: &NullLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#undefinedLiteral}.
 * @param ctx the parse tree
 */
fn enter_undefinedLiteral(&mut self, _ctx: &UndefinedLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#undefinedLiteral}.
 * @param ctx the parse tree
 */
fn exit_undefinedLiteral(&mut self, _ctx: &UndefinedLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#stringLiteral}.
 * @param ctx the parse tree
 */
fn enter_stringLiteral(&mut self, _ctx: &StringLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#stringLiteral}.
 * @param ctx the parse tree
 */
fn exit_stringLiteral(&mut self, _ctx: &StringLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#numberLiteral}.
 * @param ctx the parse tree
 */
fn enter_numberLiteral(&mut self, _ctx: &NumberLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#numberLiteral}.
 * @param ctx the parse tree
 */
fn exit_numberLiteral(&mut self, _ctx: &NumberLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#filterComponent}.
 * @param ctx the parse tree
 */
fn enter_filterComponent(&mut self, _ctx: &FilterComponentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#filterComponent}.
 * @param ctx the parse tree
 */
fn exit_filterComponent(&mut self, _ctx: &FilterComponentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#filterStatement}.
 * @param ctx the parse tree
 */
fn enter_filterStatement(&mut self, _ctx: &FilterStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#filterStatement}.
 * @param ctx the parse tree
 */
fn exit_filterStatement(&mut self, _ctx: &FilterStatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#signedNumber}.
 * @param ctx the parse tree
 */
fn enter_signedNumber(&mut self, _ctx: &SignedNumberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#signedNumber}.
 * @param ctx the parse tree
 */
fn exit_signedNumber(&mut self, _ctx: &SignedNumberContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#saplID}.
 * @param ctx the parse tree
 */
fn enter_saplID(&mut self, _ctx: &SaplIDContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#saplID}.
 * @param ctx the parse tree
 */
fn exit_saplID(&mut self, _ctx: &SaplIDContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SAPLParser#reservedID}.
 * @param ctx the parse tree
 */
fn enter_reservedID(&mut self, _ctx: &ReservedIDContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SAPLParser#reservedID}.
 * @param ctx the parse tree
 */
fn exit_reservedID(&mut self, _ctx: &ReservedIDContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : SAPLListener<'input> }


