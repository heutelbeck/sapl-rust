#![allow(nonstandard_style)]
// Generated from grammar/SAPL.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::saplparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link SAPLParser}.
 */
pub trait SAPLVisitor<'input>: ParseTreeVisitor<'input,SAPLParserContextType>{
	/**
	 * Visit a parse tree produced by {@link SAPLParser#saplDocument}.
	 * @param ctx the parse tree
	 */
	fn visit_saplDocument(&mut self, ctx: &SaplDocumentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#importStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_importStatement(&mut self, ctx: &ImportStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#wildcardImport}.
	 * @param ctx the parse tree
	 */
	fn visit_wildcardImport(&mut self, ctx: &WildcardImportContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#libraryImport}.
	 * @param ctx the parse tree
	 */
	fn visit_libraryImport(&mut self, ctx: &LibraryImportContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#functionImport}.
	 * @param ctx the parse tree
	 */
	fn visit_functionImport(&mut self, ctx: &FunctionImportContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#schema}.
	 * @param ctx the parse tree
	 */
	fn visit_schema(&mut self, ctx: &SchemaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#policyElement}.
	 * @param ctx the parse tree
	 */
	fn visit_policyElement(&mut self, ctx: &PolicyElementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#policySet}.
	 * @param ctx the parse tree
	 */
	fn visit_policySet(&mut self, ctx: &PolicySetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#combiningAlgorithm}.
	 * @param ctx the parse tree
	 */
	fn visit_combiningAlgorithm(&mut self, ctx: &CombiningAlgorithmContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#policy}.
	 * @param ctx the parse tree
	 */
	fn visit_policy(&mut self, ctx: &PolicyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#entitlement}.
	 * @param ctx the parse tree
	 */
	fn visit_entitlement(&mut self, ctx: &EntitlementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#targetExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_targetExpression(&mut self, ctx: &TargetExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_statement(&mut self, ctx: &StatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#valueDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_valueDefinition(&mut self, ctx: &ValueDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#lazyOr}.
	 * @param ctx the parse tree
	 */
	fn visit_lazyOr(&mut self, ctx: &LazyOrContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#lazyAnd}.
	 * @param ctx the parse tree
	 */
	fn visit_lazyAnd(&mut self, ctx: &LazyAndContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#eagerOr}.
	 * @param ctx the parse tree
	 */
	fn visit_eagerOr(&mut self, ctx: &EagerOrContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#exclusiveOr}.
	 * @param ctx the parse tree
	 */
	fn visit_exclusiveOr(&mut self, ctx: &ExclusiveOrContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#eagerAnd}.
	 * @param ctx the parse tree
	 */
	fn visit_eagerAnd(&mut self, ctx: &EagerAndContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#equality}.
	 * @param ctx the parse tree
	 */
	fn visit_equality(&mut self, ctx: &EqualityContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#comparison}.
	 * @param ctx the parse tree
	 */
	fn visit_comparison(&mut self, ctx: &ComparisonContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#addition}.
	 * @param ctx the parse tree
	 */
	fn visit_addition(&mut self, ctx: &AdditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#multiplication}.
	 * @param ctx the parse tree
	 */
	fn visit_multiplication(&mut self, ctx: &MultiplicationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#unaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#basicExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_basicExpression(&mut self, ctx: &BasicExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#basic}.
	 * @param ctx the parse tree
	 */
	fn visit_basic(&mut self, ctx: &BasicContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#arguments}.
	 * @param ctx the parse tree
	 */
	fn visit_arguments(&mut self, ctx: &ArgumentsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#step}.
	 * @param ctx the parse tree
	 */
	fn visit_step(&mut self, ctx: &StepContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#subscript}.
	 * @param ctx the parse tree
	 */
	fn visit_subscript(&mut self, ctx: &SubscriptContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#value}.
	 * @param ctx the parse tree
	 */
	fn visit_value(&mut self, ctx: &ValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#object}.
	 * @param ctx the parse tree
	 */
	fn visit_object(&mut self, ctx: &ObjectContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#pair}.
	 * @param ctx the parse tree
	 */
	fn visit_pair(&mut self, ctx: &PairContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#array}.
	 * @param ctx the parse tree
	 */
	fn visit_array(&mut self, ctx: &ArrayContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#booleanLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_booleanLiteral(&mut self, ctx: &BooleanLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#nullLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_nullLiteral(&mut self, ctx: &NullLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#undefinedLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_undefinedLiteral(&mut self, ctx: &UndefinedLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#stringLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_stringLiteral(&mut self, ctx: &StringLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#numberLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_numberLiteral(&mut self, ctx: &NumberLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#filterComponent}.
	 * @param ctx the parse tree
	 */
	fn visit_filterComponent(&mut self, ctx: &FilterComponentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#filterStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_filterStatement(&mut self, ctx: &FilterStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#signedNumber}.
	 * @param ctx the parse tree
	 */
	fn visit_signedNumber(&mut self, ctx: &SignedNumberContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#saplID}.
	 * @param ctx the parse tree
	 */
	fn visit_saplID(&mut self, ctx: &SaplIDContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SAPLParser#reservedID}.
	 * @param ctx the parse tree
	 */
	fn visit_reservedID(&mut self, ctx: &ReservedIDContext<'input>) { self.visit_children(ctx) }

}

pub trait SAPLVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= SAPLParserContextType>{
	/**
	 * Visit a parse tree produced by {@link SAPLParser#saplDocument}.
	 * @param ctx the parse tree
	 */
		fn visit_saplDocument(&mut self, ctx: &SaplDocumentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#importStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_importStatement(&mut self, ctx: &ImportStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#wildcardImport}.
	 * @param ctx the parse tree
	 */
		fn visit_wildcardImport(&mut self, ctx: &WildcardImportContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#libraryImport}.
	 * @param ctx the parse tree
	 */
		fn visit_libraryImport(&mut self, ctx: &LibraryImportContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#functionImport}.
	 * @param ctx the parse tree
	 */
		fn visit_functionImport(&mut self, ctx: &FunctionImportContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#schema}.
	 * @param ctx the parse tree
	 */
		fn visit_schema(&mut self, ctx: &SchemaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#policyElement}.
	 * @param ctx the parse tree
	 */
		fn visit_policyElement(&mut self, ctx: &PolicyElementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#policySet}.
	 * @param ctx the parse tree
	 */
		fn visit_policySet(&mut self, ctx: &PolicySetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#combiningAlgorithm}.
	 * @param ctx the parse tree
	 */
		fn visit_combiningAlgorithm(&mut self, ctx: &CombiningAlgorithmContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#policy}.
	 * @param ctx the parse tree
	 */
		fn visit_policy(&mut self, ctx: &PolicyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#entitlement}.
	 * @param ctx the parse tree
	 */
		fn visit_entitlement(&mut self, ctx: &EntitlementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#targetExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_targetExpression(&mut self, ctx: &TargetExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_statement(&mut self, ctx: &StatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#valueDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_valueDefinition(&mut self, ctx: &ValueDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#lazyOr}.
	 * @param ctx the parse tree
	 */
		fn visit_lazyOr(&mut self, ctx: &LazyOrContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#lazyAnd}.
	 * @param ctx the parse tree
	 */
		fn visit_lazyAnd(&mut self, ctx: &LazyAndContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#eagerOr}.
	 * @param ctx the parse tree
	 */
		fn visit_eagerOr(&mut self, ctx: &EagerOrContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#exclusiveOr}.
	 * @param ctx the parse tree
	 */
		fn visit_exclusiveOr(&mut self, ctx: &ExclusiveOrContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#eagerAnd}.
	 * @param ctx the parse tree
	 */
		fn visit_eagerAnd(&mut self, ctx: &EagerAndContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#equality}.
	 * @param ctx the parse tree
	 */
		fn visit_equality(&mut self, ctx: &EqualityContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#comparison}.
	 * @param ctx the parse tree
	 */
		fn visit_comparison(&mut self, ctx: &ComparisonContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#addition}.
	 * @param ctx the parse tree
	 */
		fn visit_addition(&mut self, ctx: &AdditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#multiplication}.
	 * @param ctx the parse tree
	 */
		fn visit_multiplication(&mut self, ctx: &MultiplicationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#unaryExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#basicExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_basicExpression(&mut self, ctx: &BasicExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#basic}.
	 * @param ctx the parse tree
	 */
		fn visit_basic(&mut self, ctx: &BasicContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#arguments}.
	 * @param ctx the parse tree
	 */
		fn visit_arguments(&mut self, ctx: &ArgumentsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#step}.
	 * @param ctx the parse tree
	 */
		fn visit_step(&mut self, ctx: &StepContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#subscript}.
	 * @param ctx the parse tree
	 */
		fn visit_subscript(&mut self, ctx: &SubscriptContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#value}.
	 * @param ctx the parse tree
	 */
		fn visit_value(&mut self, ctx: &ValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#object}.
	 * @param ctx the parse tree
	 */
		fn visit_object(&mut self, ctx: &ObjectContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#pair}.
	 * @param ctx the parse tree
	 */
		fn visit_pair(&mut self, ctx: &PairContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#array}.
	 * @param ctx the parse tree
	 */
		fn visit_array(&mut self, ctx: &ArrayContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#booleanLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_booleanLiteral(&mut self, ctx: &BooleanLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#nullLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_nullLiteral(&mut self, ctx: &NullLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#undefinedLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_undefinedLiteral(&mut self, ctx: &UndefinedLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#stringLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_stringLiteral(&mut self, ctx: &StringLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#numberLiteral}.
	 * @param ctx the parse tree
	 */
		fn visit_numberLiteral(&mut self, ctx: &NumberLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#filterComponent}.
	 * @param ctx the parse tree
	 */
		fn visit_filterComponent(&mut self, ctx: &FilterComponentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#filterStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_filterStatement(&mut self, ctx: &FilterStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#signedNumber}.
	 * @param ctx the parse tree
	 */
		fn visit_signedNumber(&mut self, ctx: &SignedNumberContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#saplID}.
	 * @param ctx the parse tree
	 */
		fn visit_saplID(&mut self, ctx: &SaplIDContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SAPLParser#reservedID}.
	 * @param ctx the parse tree
	 */
		fn visit_reservedID(&mut self, ctx: &ReservedIDContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> SAPLVisitor<'input> for T
where
	T: SAPLVisitorCompat<'input>
{
	fn visit_saplDocument(&mut self, ctx: &SaplDocumentContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_saplDocument(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_importStatement(&mut self, ctx: &ImportStatementContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_importStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_wildcardImport(&mut self, ctx: &WildcardImportContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_wildcardImport(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_libraryImport(&mut self, ctx: &LibraryImportContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_libraryImport(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionImport(&mut self, ctx: &FunctionImportContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_functionImport(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_schema(&mut self, ctx: &SchemaContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_schema(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_policyElement(&mut self, ctx: &PolicyElementContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_policyElement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_policySet(&mut self, ctx: &PolicySetContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_policySet(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_combiningAlgorithm(&mut self, ctx: &CombiningAlgorithmContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_combiningAlgorithm(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_policy(&mut self, ctx: &PolicyContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_policy(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_entitlement(&mut self, ctx: &EntitlementContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_entitlement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_targetExpression(&mut self, ctx: &TargetExpressionContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_targetExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_statement(&mut self, ctx: &StatementContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_valueDefinition(&mut self, ctx: &ValueDefinitionContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_valueDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lazyOr(&mut self, ctx: &LazyOrContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_lazyOr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lazyAnd(&mut self, ctx: &LazyAndContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_lazyAnd(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_eagerOr(&mut self, ctx: &EagerOrContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_eagerOr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_exclusiveOr(&mut self, ctx: &ExclusiveOrContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_exclusiveOr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_eagerAnd(&mut self, ctx: &EagerAndContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_eagerAnd(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_equality(&mut self, ctx: &EqualityContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_equality(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_comparison(&mut self, ctx: &ComparisonContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_comparison(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_addition(&mut self, ctx: &AdditionContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_addition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multiplication(&mut self, ctx: &MultiplicationContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_multiplication(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_unaryExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_basicExpression(&mut self, ctx: &BasicExpressionContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_basicExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_basic(&mut self, ctx: &BasicContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_basic(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_arguments(&mut self, ctx: &ArgumentsContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_arguments(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_step(&mut self, ctx: &StepContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_step(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_subscript(&mut self, ctx: &SubscriptContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_subscript(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_value(&mut self, ctx: &ValueContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_value(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_object(&mut self, ctx: &ObjectContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_object(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pair(&mut self, ctx: &PairContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_pair(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_array(&mut self, ctx: &ArrayContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_array(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_booleanLiteral(&mut self, ctx: &BooleanLiteralContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_booleanLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nullLiteral(&mut self, ctx: &NullLiteralContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_nullLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_undefinedLiteral(&mut self, ctx: &UndefinedLiteralContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_undefinedLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringLiteral(&mut self, ctx: &StringLiteralContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_stringLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_numberLiteral(&mut self, ctx: &NumberLiteralContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_numberLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_filterComponent(&mut self, ctx: &FilterComponentContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_filterComponent(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_filterStatement(&mut self, ctx: &FilterStatementContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_filterStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_signedNumber(&mut self, ctx: &SignedNumberContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_signedNumber(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_saplID(&mut self, ctx: &SaplIDContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_saplID(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_reservedID(&mut self, ctx: &ReservedIDContext<'input>){
		let result = <Self as SAPLVisitorCompat>::visit_reservedID(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}