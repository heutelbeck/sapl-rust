// Generated from grammar/SAPL.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::sapllistener::*;
use super::saplvisitor::*;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const T__0:isize=1; 
		pub const T__1:isize=2; 
		pub const T__2:isize=3; 
		pub const T__3:isize=4; 
		pub const T__4:isize=5; 
		pub const T__5:isize=6; 
		pub const T__6:isize=7; 
		pub const T__7:isize=8; 
		pub const T__8:isize=9; 
		pub const T__9:isize=10; 
		pub const T__10:isize=11; 
		pub const T__11:isize=12; 
		pub const T__12:isize=13; 
		pub const T__13:isize=14; 
		pub const T__14:isize=15; 
		pub const T__15:isize=16; 
		pub const T__16:isize=17; 
		pub const T__17:isize=18; 
		pub const T__18:isize=19; 
		pub const T__19:isize=20; 
		pub const T__20:isize=21; 
		pub const T__21:isize=22; 
		pub const T__22:isize=23; 
		pub const T__23:isize=24; 
		pub const T__24:isize=25; 
		pub const T__25:isize=26; 
		pub const T__26:isize=27; 
		pub const T__27:isize=28; 
		pub const T__28:isize=29; 
		pub const T__29:isize=30; 
		pub const T__30:isize=31; 
		pub const T__31:isize=32; 
		pub const T__32:isize=33; 
		pub const T__33:isize=34; 
		pub const T__34:isize=35; 
		pub const T__35:isize=36; 
		pub const T__36:isize=37; 
		pub const T__37:isize=38; 
		pub const T__38:isize=39; 
		pub const T__39:isize=40; 
		pub const T__40:isize=41; 
		pub const T__41:isize=42; 
		pub const T__42:isize=43; 
		pub const T__43:isize=44; 
		pub const T__44:isize=45; 
		pub const T__45:isize=46; 
		pub const T__46:isize=47; 
		pub const T__47:isize=48; 
		pub const T__48:isize=49; 
		pub const T__49:isize=50; 
		pub const T__50:isize=51; 
		pub const T__51:isize=52; 
		pub const T__52:isize=53; 
		pub const T__53:isize=54; 
		pub const T__54:isize=55; 
		pub const T__55:isize=56; 
		pub const T__56:isize=57; 
		pub const T__57:isize=58; 
		pub const T__58:isize=59; 
		pub const T__59:isize=60; 
		pub const T__60:isize=61; 
		pub const T__61:isize=62; 
		pub const T__62:isize=63; 
		pub const ID:isize=64; 
		pub const FILTER:isize=65; 
		pub const SUBTEMPLATE:isize=66; 
		pub const JSONNUMBER:isize=67; 
		pub const WS:isize=68; 
		pub const ML_COMMENT:isize=69; 
		pub const SL_COMMENT:isize=70; 
		pub const STRING:isize=71;
	pub const RULE_saplDocument:usize = 0; 
	pub const RULE_importStatement:usize = 1; 
	pub const RULE_wildcardImport:usize = 2; 
	pub const RULE_libraryImport:usize = 3; 
	pub const RULE_functionImport:usize = 4; 
	pub const RULE_schema:usize = 5; 
	pub const RULE_policyElement:usize = 6; 
	pub const RULE_policySet:usize = 7; 
	pub const RULE_combiningAlgorithm:usize = 8; 
	pub const RULE_policy:usize = 9; 
	pub const RULE_entitlement:usize = 10; 
	pub const RULE_targetExpression:usize = 11; 
	pub const RULE_statement:usize = 12; 
	pub const RULE_valueDefinition:usize = 13; 
	pub const RULE_expression:usize = 14; 
	pub const RULE_lazyOr:usize = 15; 
	pub const RULE_lazyAnd:usize = 16; 
	pub const RULE_eagerOr:usize = 17; 
	pub const RULE_exclusiveOr:usize = 18; 
	pub const RULE_eagerAnd:usize = 19; 
	pub const RULE_equality:usize = 20; 
	pub const RULE_comparison:usize = 21; 
	pub const RULE_addition:usize = 22; 
	pub const RULE_multiplication:usize = 23; 
	pub const RULE_unaryExpression:usize = 24; 
	pub const RULE_basicExpression:usize = 25; 
	pub const RULE_basic:usize = 26; 
	pub const RULE_arguments:usize = 27; 
	pub const RULE_step:usize = 28; 
	pub const RULE_subscript:usize = 29; 
	pub const RULE_value:usize = 30; 
	pub const RULE_object:usize = 31; 
	pub const RULE_pair:usize = 32; 
	pub const RULE_array:usize = 33; 
	pub const RULE_booleanLiteral:usize = 34; 
	pub const RULE_nullLiteral:usize = 35; 
	pub const RULE_undefinedLiteral:usize = 36; 
	pub const RULE_stringLiteral:usize = 37; 
	pub const RULE_numberLiteral:usize = 38; 
	pub const RULE_filterComponent:usize = 39; 
	pub const RULE_filterStatement:usize = 40; 
	pub const RULE_signedNumber:usize = 41; 
	pub const RULE_saplID:usize = 42; 
	pub const RULE_reservedID:usize = 43;
	pub const ruleNames: [&'static str; 44] =  [
		"saplDocument", "importStatement", "wildcardImport", "libraryImport", 
		"functionImport", "schema", "policyElement", "policySet", "combiningAlgorithm", 
		"policy", "entitlement", "targetExpression", "statement", "valueDefinition", 
		"expression", "lazyOr", "lazyAnd", "eagerOr", "exclusiveOr", "eagerAnd", 
		"equality", "comparison", "addition", "multiplication", "unaryExpression", 
		"basicExpression", "basic", "arguments", "step", "subscript", "value", 
		"object", "pair", "array", "booleanLiteral", "nullLiteral", "undefinedLiteral", 
		"stringLiteral", "numberLiteral", "filterComponent", "filterStatement", 
		"signedNumber", "saplID", "reservedID"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;67] = [
		None, Some("'import'"), Some("'.'"), Some("'*'"), Some("'as'"), Some("'enforced'"), 
		Some("'schema'"), Some("'set'"), Some("'for'"), Some("';'"), Some("'deny-overrides'"), 
		Some("'permit-overrides'"), Some("'first-applicable'"), Some("'only-one-applicable'"), 
		Some("'deny-unless-permit'"), Some("'permit-unless-deny'"), Some("'policy'"), 
		Some("'where'"), Some("'obligation'"), Some("'advice'"), Some("'transform'"), 
		Some("'permit'"), Some("'deny'"), Some("'var'"), Some("'='"), Some("','"), 
		Some("'||'"), Some("'&&'"), Some("'|'"), Some("'^'"), Some("'&'"), Some("'=='"), 
		Some("'!='"), Some("'=~'"), Some("'<'"), Some("'<='"), Some("'>'"), Some("'>='"), 
		Some("'in'"), Some("'+'"), Some("'-'"), Some("'/'"), Some("'%'"), Some("'!'"), 
		Some("'('"), Some("')'"), Some("'|<'"), Some("'@'"), Some("'['"), Some("']'"), 
		Some("'..'"), Some("':'"), Some("'?'"), Some("'{'"), Some("'}'"), Some("'true'"), 
		Some("'false'"), Some("'null'"), Some("'undefined'"), Some("'each'"), 
		Some("'subject'"), Some("'action'"), Some("'resource'"), Some("'environment'"), 
		None, Some("'|-'"), Some("'::'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;72]  = [
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, Some("ID"), Some("FILTER"), Some("SUBTEMPLATE"), 
		Some("JSONNUMBER"), Some("WS"), Some("ML_COMMENT"), Some("SL_COMMENT"), 
		Some("STRING")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,SAPLParserExt<'input>, I, SAPLParserContextType , dyn SAPLListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type SAPLTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, SAPLParserContextType , dyn SAPLListener<'input> + 'a>;

/// Parser for SAPL grammar
pub struct SAPLParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				SAPLParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> SAPLParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> SAPLParser<'input, I, DefaultErrorStrategy<'input,SAPLParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for SAPLParser
pub trait SAPLParserContext<'input>:
	for<'x> Listenable<dyn SAPLListener<'input> + 'x > + 
	for<'x> Visitable<dyn SAPLVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=SAPLParserContextType>
{}

antlr_rust::coerce_from!{ 'input : SAPLParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn SAPLParserContext<'input> + 'input
where
    T: SAPLVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn SAPLVisitor<'input> + 'x))
    }
}

impl<'input> SAPLParserContext<'input> for TerminalNode<'input,SAPLParserContextType> {}
impl<'input> SAPLParserContext<'input> for ErrorNode<'input,SAPLParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn SAPLParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn SAPLListener<'input> + 'input }

pub struct SAPLParserContextType;
antlr_rust::tid!{SAPLParserContextType}

impl<'input> ParserNodeType<'input> for SAPLParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn SAPLParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct SAPLParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> SAPLParserExt<'input>{
}
antlr_rust::tid! { SAPLParserExt<'a> }

impl<'input> TokenAware<'input> for SAPLParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for SAPLParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for SAPLParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "SAPL.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- saplDocument ----------------
pub type SaplDocumentContextAll<'input> = SaplDocumentContext<'input>;


pub type SaplDocumentContext<'input> = BaseParserRuleContext<'input,SaplDocumentContextExt<'input>>;

#[derive(Clone)]
pub struct SaplDocumentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for SaplDocumentContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for SaplDocumentContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_saplDocument(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_saplDocument(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for SaplDocumentContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_saplDocument(self);
	}
}

impl<'input> CustomRuleContext<'input> for SaplDocumentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_saplDocument }
	//fn type_rule_index() -> usize where Self: Sized { RULE_saplDocument }
}
antlr_rust::tid!{SaplDocumentContextExt<'a>}

impl<'input> SaplDocumentContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SaplDocumentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SaplDocumentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SaplDocumentContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<SaplDocumentContextExt<'input>>{

fn policyElement(&self) -> Option<Rc<PolicyElementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,SAPLParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn importStatement_all(&self) ->  Vec<Rc<ImportStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn importStatement(&self, i: usize) -> Option<Rc<ImportStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn schema_all(&self) ->  Vec<Rc<SchemaContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn schema(&self, i: usize) -> Option<Rc<SchemaContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> SaplDocumentContextAttrs<'input> for SaplDocumentContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn saplDocument(&mut self,)
	-> Result<Rc<SaplDocumentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SaplDocumentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_saplDocument);
        let mut _localctx: Rc<SaplDocumentContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(91);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__0 {
				{
				{
				/*InvokeRule importStatement*/
				recog.base.set_state(88);
				recog.importStatement()?;

				}
				}
				recog.base.set_state(93);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(97);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 60)) & !0x3f) == 0 && ((1usize << (_la - 60)) & ((1usize << (T__59 - 60)) | (1usize << (T__60 - 60)) | (1usize << (T__61 - 60)) | (1usize << (T__62 - 60)))) != 0) {
				{
				{
				/*InvokeRule schema*/
				recog.base.set_state(94);
				recog.schema()?;

				}
				}
				recog.base.set_state(99);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			/*InvokeRule policyElement*/
			recog.base.set_state(100);
			recog.policyElement()?;

			recog.base.set_state(101);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- importStatement ----------------
pub type ImportStatementContextAll<'input> = ImportStatementContext<'input>;


pub type ImportStatementContext<'input> = BaseParserRuleContext<'input,ImportStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ImportStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for ImportStatementContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for ImportStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_importStatement(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_importStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for ImportStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_importStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importStatement }
}
antlr_rust::tid!{ImportStatementContextExt<'a>}

impl<'input> ImportStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImportStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImportStatementContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<ImportStatementContextExt<'input>>{

fn wildcardImport(&self) -> Option<Rc<WildcardImportContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn libraryImport(&self) -> Option<Rc<LibraryImportContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn functionImport(&self) -> Option<Rc<FunctionImportContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ImportStatementContextAttrs<'input> for ImportStatementContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn importStatement(&mut self,)
	-> Result<Rc<ImportStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ImportStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_importStatement);
        let mut _localctx: Rc<ImportStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(103);
			recog.base.match_token(T__0,&mut recog.err_handler)?;

			recog.base.set_state(107);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(2,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule wildcardImport*/
					recog.base.set_state(104);
					recog.wildcardImport()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule libraryImport*/
					recog.base.set_state(105);
					recog.libraryImport()?;

					}
				}
			,
				3 =>{
					{
					/*InvokeRule functionImport*/
					recog.base.set_state(106);
					recog.functionImport()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- wildcardImport ----------------
pub type WildcardImportContextAll<'input> = WildcardImportContext<'input>;


pub type WildcardImportContext<'input> = BaseParserRuleContext<'input,WildcardImportContextExt<'input>>;

#[derive(Clone)]
pub struct WildcardImportContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for WildcardImportContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for WildcardImportContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_wildcardImport(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_wildcardImport(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for WildcardImportContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_wildcardImport(self);
	}
}

impl<'input> CustomRuleContext<'input> for WildcardImportContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_wildcardImport }
	//fn type_rule_index() -> usize where Self: Sized { RULE_wildcardImport }
}
antlr_rust::tid!{WildcardImportContextExt<'a>}

impl<'input> WildcardImportContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<WildcardImportContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,WildcardImportContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait WildcardImportContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<WildcardImportContextExt<'input>>{

fn saplID_all(&self) ->  Vec<Rc<SaplIDContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn saplID(&self, i: usize) -> Option<Rc<SaplIDContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> WildcardImportContextAttrs<'input> for WildcardImportContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn wildcardImport(&mut self,)
	-> Result<Rc<WildcardImportContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = WildcardImportContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_wildcardImport);
        let mut _localctx: Rc<WildcardImportContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule saplID*/
			recog.base.set_state(109);
			recog.saplID()?;

			recog.base.set_state(114);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(3,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(110);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					/*InvokeRule saplID*/
					recog.base.set_state(111);
					recog.saplID()?;

					}
					} 
				}
				recog.base.set_state(116);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(3,&mut recog.base)?;
			}
			recog.base.set_state(117);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			recog.base.set_state(118);
			recog.base.match_token(T__2,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- libraryImport ----------------
pub type LibraryImportContextAll<'input> = LibraryImportContext<'input>;


pub type LibraryImportContext<'input> = BaseParserRuleContext<'input,LibraryImportContextExt<'input>>;

#[derive(Clone)]
pub struct LibraryImportContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for LibraryImportContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for LibraryImportContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_libraryImport(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_libraryImport(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for LibraryImportContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_libraryImport(self);
	}
}

impl<'input> CustomRuleContext<'input> for LibraryImportContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_libraryImport }
	//fn type_rule_index() -> usize where Self: Sized { RULE_libraryImport }
}
antlr_rust::tid!{LibraryImportContextExt<'a>}

impl<'input> LibraryImportContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LibraryImportContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LibraryImportContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LibraryImportContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<LibraryImportContextExt<'input>>{

fn saplID_all(&self) ->  Vec<Rc<SaplIDContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn saplID(&self, i: usize) -> Option<Rc<SaplIDContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> LibraryImportContextAttrs<'input> for LibraryImportContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn libraryImport(&mut self,)
	-> Result<Rc<LibraryImportContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LibraryImportContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_libraryImport);
        let mut _localctx: Rc<LibraryImportContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule saplID*/
			recog.base.set_state(120);
			recog.saplID()?;

			recog.base.set_state(125);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__1 {
				{
				{
				recog.base.set_state(121);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

				/*InvokeRule saplID*/
				recog.base.set_state(122);
				recog.saplID()?;

				}
				}
				recog.base.set_state(127);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(128);
			recog.base.match_token(T__3,&mut recog.err_handler)?;

			/*InvokeRule saplID*/
			recog.base.set_state(129);
			recog.saplID()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- functionImport ----------------
pub type FunctionImportContextAll<'input> = FunctionImportContext<'input>;


pub type FunctionImportContext<'input> = BaseParserRuleContext<'input,FunctionImportContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionImportContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for FunctionImportContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for FunctionImportContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_functionImport(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_functionImport(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for FunctionImportContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_functionImport(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctionImportContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functionImport }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functionImport }
}
antlr_rust::tid!{FunctionImportContextExt<'a>}

impl<'input> FunctionImportContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunctionImportContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctionImportContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FunctionImportContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<FunctionImportContextExt<'input>>{

fn saplID_all(&self) ->  Vec<Rc<SaplIDContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn saplID(&self, i: usize) -> Option<Rc<SaplIDContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> FunctionImportContextAttrs<'input> for FunctionImportContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn functionImport(&mut self,)
	-> Result<Rc<FunctionImportContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctionImportContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_functionImport);
        let mut _localctx: Rc<FunctionImportContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule saplID*/
			recog.base.set_state(131);
			recog.saplID()?;

			recog.base.set_state(136);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(5,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(132);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					/*InvokeRule saplID*/
					recog.base.set_state(133);
					recog.saplID()?;

					}
					} 
				}
				recog.base.set_state(138);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(5,&mut recog.base)?;
			}
			recog.base.set_state(139);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			/*InvokeRule saplID*/
			recog.base.set_state(140);
			recog.saplID()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- schema ----------------
pub type SchemaContextAll<'input> = SchemaContext<'input>;


pub type SchemaContext<'input> = BaseParserRuleContext<'input,SchemaContextExt<'input>>;

#[derive(Clone)]
pub struct SchemaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for SchemaContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for SchemaContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_schema(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_schema(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for SchemaContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_schema(self);
	}
}

impl<'input> CustomRuleContext<'input> for SchemaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_schema }
	//fn type_rule_index() -> usize where Self: Sized { RULE_schema }
}
antlr_rust::tid!{SchemaContextExt<'a>}

impl<'input> SchemaContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SchemaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SchemaContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SchemaContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<SchemaContextExt<'input>>{

fn reservedID(&self) -> Option<Rc<ReservedIDContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SchemaContextAttrs<'input> for SchemaContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn schema(&mut self,)
	-> Result<Rc<SchemaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SchemaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_schema);
        let mut _localctx: Rc<SchemaContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule reservedID*/
			recog.base.set_state(142);
			recog.reservedID()?;

			recog.base.set_state(144);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__4 {
				{
				recog.base.set_state(143);
				recog.base.match_token(T__4,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(146);
			recog.base.match_token(T__5,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(147);
			recog.expression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- policyElement ----------------
pub type PolicyElementContextAll<'input> = PolicyElementContext<'input>;


pub type PolicyElementContext<'input> = BaseParserRuleContext<'input,PolicyElementContextExt<'input>>;

#[derive(Clone)]
pub struct PolicyElementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for PolicyElementContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for PolicyElementContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_policyElement(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_policyElement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for PolicyElementContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_policyElement(self);
	}
}

impl<'input> CustomRuleContext<'input> for PolicyElementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_policyElement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_policyElement }
}
antlr_rust::tid!{PolicyElementContextExt<'a>}

impl<'input> PolicyElementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PolicyElementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PolicyElementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PolicyElementContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<PolicyElementContextExt<'input>>{

fn policySet(&self) -> Option<Rc<PolicySetContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn policy(&self) -> Option<Rc<PolicyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PolicyElementContextAttrs<'input> for PolicyElementContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn policyElement(&mut self,)
	-> Result<Rc<PolicyElementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PolicyElementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_policyElement);
        let mut _localctx: Rc<PolicyElementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(151);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__6 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule policySet*/
					recog.base.set_state(149);
					recog.policySet()?;

					}
				}

			 T__15 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule policy*/
					recog.base.set_state(150);
					recog.policy()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- policySet ----------------
pub type PolicySetContextAll<'input> = PolicySetContext<'input>;


pub type PolicySetContext<'input> = BaseParserRuleContext<'input,PolicySetContextExt<'input>>;

#[derive(Clone)]
pub struct PolicySetContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for PolicySetContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for PolicySetContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_policySet(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_policySet(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for PolicySetContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_policySet(self);
	}
}

impl<'input> CustomRuleContext<'input> for PolicySetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_policySet }
	//fn type_rule_index() -> usize where Self: Sized { RULE_policySet }
}
antlr_rust::tid!{PolicySetContextExt<'a>}

impl<'input> PolicySetContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PolicySetContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PolicySetContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PolicySetContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<PolicySetContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,SAPLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
fn combiningAlgorithm(&self) -> Option<Rc<CombiningAlgorithmContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn targetExpression(&self) -> Option<Rc<TargetExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn valueDefinition_all(&self) ->  Vec<Rc<ValueDefinitionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn valueDefinition(&self, i: usize) -> Option<Rc<ValueDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn policy_all(&self) ->  Vec<Rc<PolicyContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn policy(&self, i: usize) -> Option<Rc<PolicyContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> PolicySetContextAttrs<'input> for PolicySetContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn policySet(&mut self,)
	-> Result<Rc<PolicySetContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PolicySetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_policySet);
        let mut _localctx: Rc<PolicySetContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(153);
			recog.base.match_token(T__6,&mut recog.err_handler)?;

			recog.base.set_state(154);
			recog.base.match_token(STRING,&mut recog.err_handler)?;

			/*InvokeRule combiningAlgorithm*/
			recog.base.set_state(155);
			recog.combiningAlgorithm()?;

			recog.base.set_state(158);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__7 {
				{
				recog.base.set_state(156);
				recog.base.match_token(T__7,&mut recog.err_handler)?;

				/*InvokeRule targetExpression*/
				recog.base.set_state(157);
				recog.targetExpression()?;

				}
			}

			recog.base.set_state(165);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__22 {
				{
				{
				/*InvokeRule valueDefinition*/
				recog.base.set_state(160);
				recog.valueDefinition()?;

				recog.base.set_state(161);
				recog.base.match_token(T__8,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(167);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(169); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule policy*/
				recog.base.set_state(168);
				recog.policy()?;

				}
				}
				recog.base.set_state(171); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==T__15) {break}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- combiningAlgorithm ----------------
pub type CombiningAlgorithmContextAll<'input> = CombiningAlgorithmContext<'input>;


pub type CombiningAlgorithmContext<'input> = BaseParserRuleContext<'input,CombiningAlgorithmContextExt<'input>>;

#[derive(Clone)]
pub struct CombiningAlgorithmContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for CombiningAlgorithmContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for CombiningAlgorithmContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_combiningAlgorithm(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_combiningAlgorithm(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for CombiningAlgorithmContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_combiningAlgorithm(self);
	}
}

impl<'input> CustomRuleContext<'input> for CombiningAlgorithmContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_combiningAlgorithm }
	//fn type_rule_index() -> usize where Self: Sized { RULE_combiningAlgorithm }
}
antlr_rust::tid!{CombiningAlgorithmContextExt<'a>}

impl<'input> CombiningAlgorithmContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CombiningAlgorithmContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CombiningAlgorithmContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CombiningAlgorithmContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<CombiningAlgorithmContextExt<'input>>{


}

impl<'input> CombiningAlgorithmContextAttrs<'input> for CombiningAlgorithmContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn combiningAlgorithm(&mut self,)
	-> Result<Rc<CombiningAlgorithmContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CombiningAlgorithmContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_combiningAlgorithm);
        let mut _localctx: Rc<CombiningAlgorithmContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(173);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__9) | (1usize << T__10) | (1usize << T__11) | (1usize << T__12) | (1usize << T__13) | (1usize << T__14))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- policy ----------------
pub type PolicyContextAll<'input> = PolicyContext<'input>;


pub type PolicyContext<'input> = BaseParserRuleContext<'input,PolicyContextExt<'input>>;

#[derive(Clone)]
pub struct PolicyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for PolicyContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for PolicyContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_policy(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_policy(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for PolicyContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_policy(self);
	}
}

impl<'input> CustomRuleContext<'input> for PolicyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_policy }
	//fn type_rule_index() -> usize where Self: Sized { RULE_policy }
}
antlr_rust::tid!{PolicyContextExt<'a>}

impl<'input> PolicyContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PolicyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PolicyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PolicyContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<PolicyContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,SAPLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
fn entitlement(&self) -> Option<Rc<EntitlementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn targetExpression(&self) -> Option<Rc<TargetExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> PolicyContextAttrs<'input> for PolicyContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn policy(&mut self,)
	-> Result<Rc<PolicyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PolicyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_policy);
        let mut _localctx: Rc<PolicyContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(175);
			recog.base.match_token(T__15,&mut recog.err_handler)?;

			recog.base.set_state(176);
			recog.base.match_token(STRING,&mut recog.err_handler)?;

			/*InvokeRule entitlement*/
			recog.base.set_state(177);
			recog.entitlement()?;

			recog.base.set_state(179);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 34)) & !0x3f) == 0 && ((1usize << (_la - 34)) & ((1usize << (T__33 - 34)) | (1usize << (T__38 - 34)) | (1usize << (T__39 - 34)) | (1usize << (T__42 - 34)) | (1usize << (T__43 - 34)) | (1usize << (T__45 - 34)) | (1usize << (T__46 - 34)) | (1usize << (T__47 - 34)) | (1usize << (T__52 - 34)) | (1usize << (T__54 - 34)) | (1usize << (T__55 - 34)) | (1usize << (T__56 - 34)) | (1usize << (T__57 - 34)) | (1usize << (T__59 - 34)) | (1usize << (T__60 - 34)) | (1usize << (T__61 - 34)) | (1usize << (T__62 - 34)) | (1usize << (ID - 34)))) != 0) || _la==JSONNUMBER || _la==STRING {
				{
				/*InvokeRule targetExpression*/
				recog.base.set_state(178);
				recog.targetExpression()?;

				}
			}

			recog.base.set_state(189);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__16 {
				{
				recog.base.set_state(181);
				recog.base.match_token(T__16,&mut recog.err_handler)?;

				/*InvokeRule statement*/
				recog.base.set_state(182);
				recog.statement()?;

				recog.base.set_state(186);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while ((((_la - 23)) & !0x3f) == 0 && ((1usize << (_la - 23)) & ((1usize << (T__22 - 23)) | (1usize << (T__33 - 23)) | (1usize << (T__38 - 23)) | (1usize << (T__39 - 23)) | (1usize << (T__42 - 23)) | (1usize << (T__43 - 23)) | (1usize << (T__45 - 23)) | (1usize << (T__46 - 23)) | (1usize << (T__47 - 23)) | (1usize << (T__52 - 23)))) != 0) || ((((_la - 55)) & !0x3f) == 0 && ((1usize << (_la - 55)) & ((1usize << (T__54 - 55)) | (1usize << (T__55 - 55)) | (1usize << (T__56 - 55)) | (1usize << (T__57 - 55)) | (1usize << (T__59 - 55)) | (1usize << (T__60 - 55)) | (1usize << (T__61 - 55)) | (1usize << (T__62 - 55)) | (1usize << (ID - 55)) | (1usize << (JSONNUMBER - 55)) | (1usize << (STRING - 55)))) != 0) {
					{
					{
					/*InvokeRule statement*/
					recog.base.set_state(183);
					recog.statement()?;

					}
					}
					recog.base.set_state(188);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(195);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__17 {
				{
				{
				recog.base.set_state(191);
				recog.base.match_token(T__17,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(192);
				recog.expression()?;

				}
				}
				recog.base.set_state(197);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(202);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__18 {
				{
				{
				recog.base.set_state(198);
				recog.base.match_token(T__18,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(199);
				recog.expression()?;

				}
				}
				recog.base.set_state(204);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(209);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__19 {
				{
				{
				recog.base.set_state(205);
				recog.base.match_token(T__19,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(206);
				recog.expression()?;

				}
				}
				recog.base.set_state(211);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- entitlement ----------------
pub type EntitlementContextAll<'input> = EntitlementContext<'input>;


pub type EntitlementContext<'input> = BaseParserRuleContext<'input,EntitlementContextExt<'input>>;

#[derive(Clone)]
pub struct EntitlementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for EntitlementContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for EntitlementContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_entitlement(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_entitlement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for EntitlementContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_entitlement(self);
	}
}

impl<'input> CustomRuleContext<'input> for EntitlementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_entitlement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_entitlement }
}
antlr_rust::tid!{EntitlementContextExt<'a>}

impl<'input> EntitlementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EntitlementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EntitlementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EntitlementContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<EntitlementContextExt<'input>>{


}

impl<'input> EntitlementContextAttrs<'input> for EntitlementContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn entitlement(&mut self,)
	-> Result<Rc<EntitlementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EntitlementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_entitlement);
        let mut _localctx: Rc<EntitlementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(212);
			_la = recog.base.input.la(1);
			if { !(_la==T__20 || _la==T__21) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- targetExpression ----------------
pub type TargetExpressionContextAll<'input> = TargetExpressionContext<'input>;


pub type TargetExpressionContext<'input> = BaseParserRuleContext<'input,TargetExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct TargetExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for TargetExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for TargetExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_targetExpression(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_targetExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for TargetExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_targetExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for TargetExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_targetExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_targetExpression }
}
antlr_rust::tid!{TargetExpressionContextExt<'a>}

impl<'input> TargetExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TargetExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TargetExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TargetExpressionContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<TargetExpressionContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TargetExpressionContextAttrs<'input> for TargetExpressionContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn targetExpression(&mut self,)
	-> Result<Rc<TargetExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TargetExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_targetExpression);
        let mut _localctx: Rc<TargetExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(214);
			recog.expression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- statement ----------------
pub type StatementContextAll<'input> = StatementContext<'input>;


pub type StatementContext<'input> = BaseParserRuleContext<'input,StatementContextExt<'input>>;

#[derive(Clone)]
pub struct StatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for StatementContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for StatementContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_statement(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_statement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for StatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for StatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}
antlr_rust::tid!{StatementContextExt<'a>}

impl<'input> StatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StatementContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<StatementContextExt<'input>>{

fn valueDefinition(&self) -> Option<Rc<ValueDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StatementContextAttrs<'input> for StatementContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn statement(&mut self,)
	-> Result<Rc<StatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_statement);
        let mut _localctx: Rc<StatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(218);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__22 
				=> {
					{
					/*InvokeRule valueDefinition*/
					recog.base.set_state(216);
					recog.valueDefinition()?;

					}
				}

			 T__33 | T__38 | T__39 | T__42 | T__43 | T__45 | T__46 | T__47 | T__52 |
			 T__54 | T__55 | T__56 | T__57 | T__59 | T__60 | T__61 | T__62 | ID |
			 JSONNUMBER | STRING 
				=> {
					{
					/*InvokeRule expression*/
					recog.base.set_state(217);
					recog.expression()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(220);
			recog.base.match_token(T__8,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- valueDefinition ----------------
pub type ValueDefinitionContextAll<'input> = ValueDefinitionContext<'input>;


pub type ValueDefinitionContext<'input> = BaseParserRuleContext<'input,ValueDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct ValueDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for ValueDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for ValueDefinitionContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_valueDefinition(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_valueDefinition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for ValueDefinitionContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_valueDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValueDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_valueDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_valueDefinition }
}
antlr_rust::tid!{ValueDefinitionContextExt<'a>}

impl<'input> ValueDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ValueDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ValueDefinitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ValueDefinitionContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<ValueDefinitionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ID
/// Returns `None` if there is no child corresponding to token ID
fn ID(&self) -> Option<Rc<TerminalNode<'input,SAPLParserContextType>>> where Self:Sized{
	self.get_token(ID, 0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ValueDefinitionContextAttrs<'input> for ValueDefinitionContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn valueDefinition(&mut self,)
	-> Result<Rc<ValueDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ValueDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_valueDefinition);
        let mut _localctx: Rc<ValueDefinitionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(222);
			recog.base.match_token(T__22,&mut recog.err_handler)?;

			recog.base.set_state(223);
			recog.base.match_token(ID,&mut recog.err_handler)?;

			recog.base.set_state(224);
			recog.base.match_token(T__23,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(225);
			recog.expression()?;

			recog.base.set_state(235);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__5 {
				{
				recog.base.set_state(226);
				recog.base.match_token(T__5,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(227);
				recog.expression()?;

				recog.base.set_state(232);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==T__24 {
					{
					{
					recog.base.set_state(228);
					recog.base.match_token(T__24,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(229);
					recog.expression()?;

					}
					}
					recog.base.set_state(234);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expression ----------------
pub type ExpressionContextAll<'input> = ExpressionContext<'input>;


pub type ExpressionContext<'input> = BaseParserRuleContext<'input,ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for ExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expression(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_expression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for ExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::tid!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>{

fn lazyOr(&self) -> Option<Rc<LazyOrContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expression(&mut self,)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_expression);
        let mut _localctx: Rc<ExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule lazyOr*/
			recog.base.set_state(237);
			recog.lazyOr()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- lazyOr ----------------
pub type LazyOrContextAll<'input> = LazyOrContext<'input>;


pub type LazyOrContext<'input> = BaseParserRuleContext<'input,LazyOrContextExt<'input>>;

#[derive(Clone)]
pub struct LazyOrContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for LazyOrContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for LazyOrContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_lazyOr(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_lazyOr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for LazyOrContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_lazyOr(self);
	}
}

impl<'input> CustomRuleContext<'input> for LazyOrContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lazyOr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lazyOr }
}
antlr_rust::tid!{LazyOrContextExt<'a>}

impl<'input> LazyOrContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LazyOrContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LazyOrContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LazyOrContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<LazyOrContextExt<'input>>{

fn lazyAnd_all(&self) ->  Vec<Rc<LazyAndContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn lazyAnd(&self, i: usize) -> Option<Rc<LazyAndContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> LazyOrContextAttrs<'input> for LazyOrContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn lazyOr(&mut self,)
	-> Result<Rc<LazyOrContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LazyOrContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_lazyOr);
        let mut _localctx: Rc<LazyOrContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule lazyAnd*/
			recog.base.set_state(239);
			recog.lazyAnd()?;

			recog.base.set_state(244);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__25 {
				{
				{
				recog.base.set_state(240);
				recog.base.match_token(T__25,&mut recog.err_handler)?;

				/*InvokeRule lazyAnd*/
				recog.base.set_state(241);
				recog.lazyAnd()?;

				}
				}
				recog.base.set_state(246);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- lazyAnd ----------------
pub type LazyAndContextAll<'input> = LazyAndContext<'input>;


pub type LazyAndContext<'input> = BaseParserRuleContext<'input,LazyAndContextExt<'input>>;

#[derive(Clone)]
pub struct LazyAndContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for LazyAndContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for LazyAndContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_lazyAnd(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_lazyAnd(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for LazyAndContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_lazyAnd(self);
	}
}

impl<'input> CustomRuleContext<'input> for LazyAndContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lazyAnd }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lazyAnd }
}
antlr_rust::tid!{LazyAndContextExt<'a>}

impl<'input> LazyAndContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LazyAndContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LazyAndContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LazyAndContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<LazyAndContextExt<'input>>{

fn eagerOr_all(&self) ->  Vec<Rc<EagerOrContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn eagerOr(&self, i: usize) -> Option<Rc<EagerOrContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> LazyAndContextAttrs<'input> for LazyAndContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn lazyAnd(&mut self,)
	-> Result<Rc<LazyAndContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LazyAndContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_lazyAnd);
        let mut _localctx: Rc<LazyAndContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule eagerOr*/
			recog.base.set_state(247);
			recog.eagerOr()?;

			recog.base.set_state(252);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__26 {
				{
				{
				recog.base.set_state(248);
				recog.base.match_token(T__26,&mut recog.err_handler)?;

				/*InvokeRule eagerOr*/
				recog.base.set_state(249);
				recog.eagerOr()?;

				}
				}
				recog.base.set_state(254);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- eagerOr ----------------
pub type EagerOrContextAll<'input> = EagerOrContext<'input>;


pub type EagerOrContext<'input> = BaseParserRuleContext<'input,EagerOrContextExt<'input>>;

#[derive(Clone)]
pub struct EagerOrContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for EagerOrContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for EagerOrContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_eagerOr(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_eagerOr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for EagerOrContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_eagerOr(self);
	}
}

impl<'input> CustomRuleContext<'input> for EagerOrContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eagerOr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eagerOr }
}
antlr_rust::tid!{EagerOrContextExt<'a>}

impl<'input> EagerOrContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EagerOrContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EagerOrContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EagerOrContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<EagerOrContextExt<'input>>{

fn exclusiveOr_all(&self) ->  Vec<Rc<ExclusiveOrContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn exclusiveOr(&self, i: usize) -> Option<Rc<ExclusiveOrContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> EagerOrContextAttrs<'input> for EagerOrContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eagerOr(&mut self,)
	-> Result<Rc<EagerOrContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EagerOrContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_eagerOr);
        let mut _localctx: Rc<EagerOrContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule exclusiveOr*/
			recog.base.set_state(255);
			recog.exclusiveOr()?;

			recog.base.set_state(260);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__27 {
				{
				{
				recog.base.set_state(256);
				recog.base.match_token(T__27,&mut recog.err_handler)?;

				/*InvokeRule exclusiveOr*/
				recog.base.set_state(257);
				recog.exclusiveOr()?;

				}
				}
				recog.base.set_state(262);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- exclusiveOr ----------------
pub type ExclusiveOrContextAll<'input> = ExclusiveOrContext<'input>;


pub type ExclusiveOrContext<'input> = BaseParserRuleContext<'input,ExclusiveOrContextExt<'input>>;

#[derive(Clone)]
pub struct ExclusiveOrContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for ExclusiveOrContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for ExclusiveOrContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_exclusiveOr(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_exclusiveOr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for ExclusiveOrContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_exclusiveOr(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExclusiveOrContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exclusiveOr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exclusiveOr }
}
antlr_rust::tid!{ExclusiveOrContextExt<'a>}

impl<'input> ExclusiveOrContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExclusiveOrContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExclusiveOrContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExclusiveOrContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<ExclusiveOrContextExt<'input>>{

fn eagerAnd_all(&self) ->  Vec<Rc<EagerAndContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn eagerAnd(&self, i: usize) -> Option<Rc<EagerAndContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ExclusiveOrContextAttrs<'input> for ExclusiveOrContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn exclusiveOr(&mut self,)
	-> Result<Rc<ExclusiveOrContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExclusiveOrContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_exclusiveOr);
        let mut _localctx: Rc<ExclusiveOrContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule eagerAnd*/
			recog.base.set_state(263);
			recog.eagerAnd()?;

			recog.base.set_state(268);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__28 {
				{
				{
				recog.base.set_state(264);
				recog.base.match_token(T__28,&mut recog.err_handler)?;

				/*InvokeRule eagerAnd*/
				recog.base.set_state(265);
				recog.eagerAnd()?;

				}
				}
				recog.base.set_state(270);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- eagerAnd ----------------
pub type EagerAndContextAll<'input> = EagerAndContext<'input>;


pub type EagerAndContext<'input> = BaseParserRuleContext<'input,EagerAndContextExt<'input>>;

#[derive(Clone)]
pub struct EagerAndContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for EagerAndContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for EagerAndContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_eagerAnd(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_eagerAnd(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for EagerAndContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_eagerAnd(self);
	}
}

impl<'input> CustomRuleContext<'input> for EagerAndContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eagerAnd }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eagerAnd }
}
antlr_rust::tid!{EagerAndContextExt<'a>}

impl<'input> EagerAndContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EagerAndContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EagerAndContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EagerAndContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<EagerAndContextExt<'input>>{

fn equality_all(&self) ->  Vec<Rc<EqualityContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn equality(&self, i: usize) -> Option<Rc<EqualityContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> EagerAndContextAttrs<'input> for EagerAndContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eagerAnd(&mut self,)
	-> Result<Rc<EagerAndContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EagerAndContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_eagerAnd);
        let mut _localctx: Rc<EagerAndContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule equality*/
			recog.base.set_state(271);
			recog.equality()?;

			recog.base.set_state(276);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__29 {
				{
				{
				recog.base.set_state(272);
				recog.base.match_token(T__29,&mut recog.err_handler)?;

				/*InvokeRule equality*/
				recog.base.set_state(273);
				recog.equality()?;

				}
				}
				recog.base.set_state(278);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- equality ----------------
pub type EqualityContextAll<'input> = EqualityContext<'input>;


pub type EqualityContext<'input> = BaseParserRuleContext<'input,EqualityContextExt<'input>>;

#[derive(Clone)]
pub struct EqualityContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for EqualityContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for EqualityContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_equality(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_equality(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for EqualityContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_equality(self);
	}
}

impl<'input> CustomRuleContext<'input> for EqualityContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_equality }
	//fn type_rule_index() -> usize where Self: Sized { RULE_equality }
}
antlr_rust::tid!{EqualityContextExt<'a>}

impl<'input> EqualityContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EqualityContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EqualityContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EqualityContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<EqualityContextExt<'input>>{

fn comparison_all(&self) ->  Vec<Rc<ComparisonContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn comparison(&self, i: usize) -> Option<Rc<ComparisonContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> EqualityContextAttrs<'input> for EqualityContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn equality(&mut self,)
	-> Result<Rc<EqualityContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EqualityContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_equality);
        let mut _localctx: Rc<EqualityContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule comparison*/
			recog.base.set_state(279);
			recog.comparison()?;

			recog.base.set_state(282);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 31)) & !0x3f) == 0 && ((1usize << (_la - 31)) & ((1usize << (T__30 - 31)) | (1usize << (T__31 - 31)) | (1usize << (T__32 - 31)))) != 0) {
				{
				recog.base.set_state(280);
				_la = recog.base.input.la(1);
				if { !(((((_la - 31)) & !0x3f) == 0 && ((1usize << (_la - 31)) & ((1usize << (T__30 - 31)) | (1usize << (T__31 - 31)) | (1usize << (T__32 - 31)))) != 0)) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				/*InvokeRule comparison*/
				recog.base.set_state(281);
				recog.comparison()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- comparison ----------------
pub type ComparisonContextAll<'input> = ComparisonContext<'input>;


pub type ComparisonContext<'input> = BaseParserRuleContext<'input,ComparisonContextExt<'input>>;

#[derive(Clone)]
pub struct ComparisonContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for ComparisonContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for ComparisonContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_comparison(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_comparison(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for ComparisonContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_comparison(self);
	}
}

impl<'input> CustomRuleContext<'input> for ComparisonContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comparison }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comparison }
}
antlr_rust::tid!{ComparisonContextExt<'a>}

impl<'input> ComparisonContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ComparisonContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ComparisonContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ComparisonContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<ComparisonContextExt<'input>>{

fn addition_all(&self) ->  Vec<Rc<AdditionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn addition(&self, i: usize) -> Option<Rc<AdditionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ComparisonContextAttrs<'input> for ComparisonContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn comparison(&mut self,)
	-> Result<Rc<ComparisonContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ComparisonContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_comparison);
        let mut _localctx: Rc<ComparisonContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule addition*/
			recog.base.set_state(284);
			recog.addition()?;

			recog.base.set_state(287);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 34)) & !0x3f) == 0 && ((1usize << (_la - 34)) & ((1usize << (T__33 - 34)) | (1usize << (T__34 - 34)) | (1usize << (T__35 - 34)) | (1usize << (T__36 - 34)) | (1usize << (T__37 - 34)))) != 0) {
				{
				recog.base.set_state(285);
				_la = recog.base.input.la(1);
				if { !(((((_la - 34)) & !0x3f) == 0 && ((1usize << (_la - 34)) & ((1usize << (T__33 - 34)) | (1usize << (T__34 - 34)) | (1usize << (T__35 - 34)) | (1usize << (T__36 - 34)) | (1usize << (T__37 - 34)))) != 0)) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				/*InvokeRule addition*/
				recog.base.set_state(286);
				recog.addition()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- addition ----------------
pub type AdditionContextAll<'input> = AdditionContext<'input>;


pub type AdditionContext<'input> = BaseParserRuleContext<'input,AdditionContextExt<'input>>;

#[derive(Clone)]
pub struct AdditionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for AdditionContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for AdditionContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_addition(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_addition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for AdditionContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_addition(self);
	}
}

impl<'input> CustomRuleContext<'input> for AdditionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_addition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_addition }
}
antlr_rust::tid!{AdditionContextExt<'a>}

impl<'input> AdditionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AdditionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AdditionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AdditionContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<AdditionContextExt<'input>>{

fn multiplication_all(&self) ->  Vec<Rc<MultiplicationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn multiplication(&self, i: usize) -> Option<Rc<MultiplicationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> AdditionContextAttrs<'input> for AdditionContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn addition(&mut self,)
	-> Result<Rc<AdditionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AdditionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_addition);
        let mut _localctx: Rc<AdditionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule multiplication*/
			recog.base.set_state(289);
			recog.multiplication()?;

			recog.base.set_state(294);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__38 || _la==T__39 {
				{
				{
				recog.base.set_state(290);
				_la = recog.base.input.la(1);
				if { !(_la==T__38 || _la==T__39) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				/*InvokeRule multiplication*/
				recog.base.set_state(291);
				recog.multiplication()?;

				}
				}
				recog.base.set_state(296);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- multiplication ----------------
pub type MultiplicationContextAll<'input> = MultiplicationContext<'input>;


pub type MultiplicationContext<'input> = BaseParserRuleContext<'input,MultiplicationContextExt<'input>>;

#[derive(Clone)]
pub struct MultiplicationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for MultiplicationContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for MultiplicationContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_multiplication(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_multiplication(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for MultiplicationContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_multiplication(self);
	}
}

impl<'input> CustomRuleContext<'input> for MultiplicationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_multiplication }
	//fn type_rule_index() -> usize where Self: Sized { RULE_multiplication }
}
antlr_rust::tid!{MultiplicationContextExt<'a>}

impl<'input> MultiplicationContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MultiplicationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MultiplicationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MultiplicationContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<MultiplicationContextExt<'input>>{

fn unaryExpression_all(&self) ->  Vec<Rc<UnaryExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn unaryExpression(&self, i: usize) -> Option<Rc<UnaryExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> MultiplicationContextAttrs<'input> for MultiplicationContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn multiplication(&mut self,)
	-> Result<Rc<MultiplicationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MultiplicationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_multiplication);
        let mut _localctx: Rc<MultiplicationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule unaryExpression*/
			recog.base.set_state(297);
			recog.unaryExpression()?;

			recog.base.set_state(302);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__2 || _la==T__40 || _la==T__41 {
				{
				{
				recog.base.set_state(298);
				_la = recog.base.input.la(1);
				if { !(_la==T__2 || _la==T__40 || _la==T__41) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				/*InvokeRule unaryExpression*/
				recog.base.set_state(299);
				recog.unaryExpression()?;

				}
				}
				recog.base.set_state(304);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- unaryExpression ----------------
pub type UnaryExpressionContextAll<'input> = UnaryExpressionContext<'input>;


pub type UnaryExpressionContext<'input> = BaseParserRuleContext<'input,UnaryExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct UnaryExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for UnaryExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for UnaryExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_unaryExpression(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_unaryExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for UnaryExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_unaryExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for UnaryExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unaryExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unaryExpression }
}
antlr_rust::tid!{UnaryExpressionContextExt<'a>}

impl<'input> UnaryExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<UnaryExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UnaryExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait UnaryExpressionContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<UnaryExpressionContextExt<'input>>{

fn unaryExpression(&self) -> Option<Rc<UnaryExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn basicExpression(&self) -> Option<Rc<BasicExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> UnaryExpressionContextAttrs<'input> for UnaryExpressionContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn unaryExpression(&mut self,)
	-> Result<Rc<UnaryExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UnaryExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_unaryExpression);
        let mut _localctx: Rc<UnaryExpressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(308);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__38 | T__39 | T__42 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(305);
					_la = recog.base.input.la(1);
					if { !(((((_la - 39)) & !0x3f) == 0 && ((1usize << (_la - 39)) & ((1usize << (T__38 - 39)) | (1usize << (T__39 - 39)) | (1usize << (T__42 - 39)))) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule unaryExpression*/
					recog.base.set_state(306);
					recog.unaryExpression()?;

					}
				}

			 T__33 | T__43 | T__45 | T__46 | T__47 | T__52 | T__54 | T__55 | T__56 |
			 T__57 | T__59 | T__60 | T__61 | T__62 | ID | JSONNUMBER | STRING 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule basicExpression*/
					recog.base.set_state(307);
					recog.basicExpression()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- basicExpression ----------------
pub type BasicExpressionContextAll<'input> = BasicExpressionContext<'input>;


pub type BasicExpressionContext<'input> = BaseParserRuleContext<'input,BasicExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct BasicExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for BasicExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for BasicExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_basicExpression(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_basicExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for BasicExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_basicExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for BasicExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_basicExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_basicExpression }
}
antlr_rust::tid!{BasicExpressionContextExt<'a>}

impl<'input> BasicExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BasicExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BasicExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BasicExpressionContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<BasicExpressionContextExt<'input>>{

fn basic(&self) -> Option<Rc<BasicContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token FILTER
/// Returns `None` if there is no child corresponding to token FILTER
fn FILTER(&self) -> Option<Rc<TerminalNode<'input,SAPLParserContextType>>> where Self:Sized{
	self.get_token(FILTER, 0)
}
fn filterComponent(&self) -> Option<Rc<FilterComponentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SUBTEMPLATE
/// Returns `None` if there is no child corresponding to token SUBTEMPLATE
fn SUBTEMPLATE(&self) -> Option<Rc<TerminalNode<'input,SAPLParserContextType>>> where Self:Sized{
	self.get_token(SUBTEMPLATE, 0)
}
fn basicExpression(&self) -> Option<Rc<BasicExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> BasicExpressionContextAttrs<'input> for BasicExpressionContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn basicExpression(&mut self,)
	-> Result<Rc<BasicExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BasicExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_basicExpression);
        let mut _localctx: Rc<BasicExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule basic*/
			recog.base.set_state(310);
			recog.basic()?;

			recog.base.set_state(315);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 FILTER 
				=> {
			    	{
			    	recog.base.set_state(311);
			    	recog.base.match_token(FILTER,&mut recog.err_handler)?;

			    	/*InvokeRule filterComponent*/
			    	recog.base.set_state(312);
			    	recog.filterComponent()?;

			    	}
			    }

			 SUBTEMPLATE 
				=> {
			    	{
			    	recog.base.set_state(313);
			    	recog.base.match_token(SUBTEMPLATE,&mut recog.err_handler)?;

			    	/*InvokeRule basicExpression*/
			    	recog.base.set_state(314);
			    	recog.basicExpression()?;

			    	}
			    }

			 EOF | T__2 | T__5 | T__6 | T__8 | T__15 | T__16 | T__17 | T__18 | T__19 |
			 T__22 | T__24 | T__25 | T__26 | T__27 | T__28 | T__29 | T__30 | T__31 |
			 T__32 | T__33 | T__34 | T__35 | T__36 | T__37 | T__38 | T__39 | T__40 |
			 T__41 | T__44 | T__48 | T__53 | T__59 | T__60 | T__61 | T__62 
				=> {
			    }

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- basic ----------------
pub type BasicContextAll<'input> = BasicContext<'input>;


pub type BasicContext<'input> = BaseParserRuleContext<'input,BasicContextExt<'input>>;

#[derive(Clone)]
pub struct BasicContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for BasicContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for BasicContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_basic(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_basic(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for BasicContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_basic(self);
	}
}

impl<'input> CustomRuleContext<'input> for BasicContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_basic }
	//fn type_rule_index() -> usize where Self: Sized { RULE_basic }
}
antlr_rust::tid!{BasicContextExt<'a>}

impl<'input> BasicContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BasicContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BasicContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BasicContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<BasicContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn step_all(&self) ->  Vec<Rc<StepContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn step(&self, i: usize) -> Option<Rc<StepContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn value(&self) -> Option<Rc<ValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn saplID_all(&self) ->  Vec<Rc<SaplIDContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn saplID(&self, i: usize) -> Option<Rc<SaplIDContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn arguments(&self) -> Option<Rc<ArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> BasicContextAttrs<'input> for BasicContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn basic(&mut self,)
	-> Result<Rc<BasicContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BasicContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_basic);
        let mut _localctx: Rc<BasicContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(400);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(43,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(317);
					recog.base.match_token(T__43,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(318);
					recog.expression()?;

					recog.base.set_state(319);
					recog.base.match_token(T__44,&mut recog.err_handler)?;

					recog.base.set_state(323);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__1 || _la==T__47 || _la==T__49 {
						{
						{
						/*InvokeRule step*/
						recog.base.set_state(320);
						recog.step()?;

						}
						}
						recog.base.set_state(325);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule value*/
					recog.base.set_state(326);
					recog.value()?;

					recog.base.set_state(330);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__1 || _la==T__47 || _la==T__49 {
						{
						{
						/*InvokeRule step*/
						recog.base.set_state(327);
						recog.step()?;

						}
						}
						recog.base.set_state(332);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule saplID*/
					recog.base.set_state(333);
					recog.saplID()?;

					recog.base.set_state(338);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__1 {
						{
						{
						recog.base.set_state(334);
						recog.base.match_token(T__1,&mut recog.err_handler)?;

						/*InvokeRule saplID*/
						recog.base.set_state(335);
						recog.saplID()?;

						}
						}
						recog.base.set_state(340);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					/*InvokeRule arguments*/
					recog.base.set_state(341);
					recog.arguments()?;

					recog.base.set_state(345);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__1 || _la==T__47 || _la==T__49 {
						{
						{
						/*InvokeRule step*/
						recog.base.set_state(342);
						recog.step()?;

						}
						}
						recog.base.set_state(347);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(348);
					recog.base.match_token(T__33,&mut recog.err_handler)?;

					/*InvokeRule saplID*/
					recog.base.set_state(349);
					recog.saplID()?;

					recog.base.set_state(354);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__1 {
						{
						{
						recog.base.set_state(350);
						recog.base.match_token(T__1,&mut recog.err_handler)?;

						/*InvokeRule saplID*/
						recog.base.set_state(351);
						recog.saplID()?;

						}
						}
						recog.base.set_state(356);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(358);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__43 {
						{
						/*InvokeRule arguments*/
						recog.base.set_state(357);
						recog.arguments()?;

						}
					}

					recog.base.set_state(360);
					recog.base.match_token(T__35,&mut recog.err_handler)?;

					recog.base.set_state(364);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__1 || _la==T__47 || _la==T__49 {
						{
						{
						/*InvokeRule step*/
						recog.base.set_state(361);
						recog.step()?;

						}
						}
						recog.base.set_state(366);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(367);
					recog.base.match_token(T__45,&mut recog.err_handler)?;

					/*InvokeRule saplID*/
					recog.base.set_state(368);
					recog.saplID()?;

					recog.base.set_state(373);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__1 {
						{
						{
						recog.base.set_state(369);
						recog.base.match_token(T__1,&mut recog.err_handler)?;

						/*InvokeRule saplID*/
						recog.base.set_state(370);
						recog.saplID()?;

						}
						}
						recog.base.set_state(375);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(377);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__43 {
						{
						/*InvokeRule arguments*/
						recog.base.set_state(376);
						recog.arguments()?;

						}
					}

					recog.base.set_state(379);
					recog.base.match_token(T__35,&mut recog.err_handler)?;

					recog.base.set_state(383);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__1 || _la==T__47 || _la==T__49 {
						{
						{
						/*InvokeRule step*/
						recog.base.set_state(380);
						recog.step()?;

						}
						}
						recog.base.set_state(385);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule saplID*/
					recog.base.set_state(386);
					recog.saplID()?;

					recog.base.set_state(390);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__1 || _la==T__47 || _la==T__49 {
						{
						{
						/*InvokeRule step*/
						recog.base.set_state(387);
						recog.step()?;

						}
						}
						recog.base.set_state(392);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					recog.base.set_state(393);
					recog.base.match_token(T__46,&mut recog.err_handler)?;

					recog.base.set_state(397);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__1 || _la==T__47 || _la==T__49 {
						{
						{
						/*InvokeRule step*/
						recog.base.set_state(394);
						recog.step()?;

						}
						}
						recog.base.set_state(399);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- arguments ----------------
pub type ArgumentsContextAll<'input> = ArgumentsContext<'input>;


pub type ArgumentsContext<'input> = BaseParserRuleContext<'input,ArgumentsContextExt<'input>>;

#[derive(Clone)]
pub struct ArgumentsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for ArgumentsContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for ArgumentsContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_arguments(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_arguments(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for ArgumentsContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_arguments(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArgumentsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arguments }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arguments }
}
antlr_rust::tid!{ArgumentsContextExt<'a>}

impl<'input> ArgumentsContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ArgumentsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArgumentsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ArgumentsContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<ArgumentsContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ArgumentsContextAttrs<'input> for ArgumentsContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn arguments(&mut self,)
	-> Result<Rc<ArgumentsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArgumentsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_arguments);
        let mut _localctx: Rc<ArgumentsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(402);
			recog.base.match_token(T__43,&mut recog.err_handler)?;

			recog.base.set_state(411);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 34)) & !0x3f) == 0 && ((1usize << (_la - 34)) & ((1usize << (T__33 - 34)) | (1usize << (T__38 - 34)) | (1usize << (T__39 - 34)) | (1usize << (T__42 - 34)) | (1usize << (T__43 - 34)) | (1usize << (T__45 - 34)) | (1usize << (T__46 - 34)) | (1usize << (T__47 - 34)) | (1usize << (T__52 - 34)) | (1usize << (T__54 - 34)) | (1usize << (T__55 - 34)) | (1usize << (T__56 - 34)) | (1usize << (T__57 - 34)) | (1usize << (T__59 - 34)) | (1usize << (T__60 - 34)) | (1usize << (T__61 - 34)) | (1usize << (T__62 - 34)) | (1usize << (ID - 34)))) != 0) || _la==JSONNUMBER || _la==STRING {
				{
				/*InvokeRule expression*/
				recog.base.set_state(403);
				recog.expression()?;

				recog.base.set_state(408);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==T__24 {
					{
					{
					recog.base.set_state(404);
					recog.base.match_token(T__24,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(405);
					recog.expression()?;

					}
					}
					recog.base.set_state(410);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(413);
			recog.base.match_token(T__44,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- step ----------------
pub type StepContextAll<'input> = StepContext<'input>;


pub type StepContext<'input> = BaseParserRuleContext<'input,StepContextExt<'input>>;

#[derive(Clone)]
pub struct StepContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for StepContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for StepContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_step(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_step(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for StepContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_step(self);
	}
}

impl<'input> CustomRuleContext<'input> for StepContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_step }
	//fn type_rule_index() -> usize where Self: Sized { RULE_step }
}
antlr_rust::tid!{StepContextExt<'a>}

impl<'input> StepContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StepContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StepContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StepContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<StepContextExt<'input>>{

fn saplID_all(&self) ->  Vec<Rc<SaplIDContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn saplID(&self, i: usize) -> Option<Rc<SaplIDContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,SAPLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
fn arguments(&self) -> Option<Rc<ArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn subscript(&self) -> Option<Rc<SubscriptContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn signedNumber(&self) -> Option<Rc<SignedNumberContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StepContextAttrs<'input> for StepContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn step(&mut self,)
	-> Result<Rc<StepContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StepContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_step);
        let mut _localctx: Rc<StepContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(472);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__1 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(415);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					recog.base.set_state(447);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 T__59 | T__60 | T__61 | T__62 | ID 
						=> {
							{
							/*InvokeRule saplID*/
							recog.base.set_state(416);
							recog.saplID()?;

							}
						}

					 STRING 
						=> {
							{
							recog.base.set_state(417);
							recog.base.match_token(STRING,&mut recog.err_handler)?;

							}
						}

					 T__2 
						=> {
							{
							recog.base.set_state(418);
							recog.base.match_token(T__2,&mut recog.err_handler)?;

							}
						}

					 T__33 
						=> {
							{
							recog.base.set_state(419);
							recog.base.match_token(T__33,&mut recog.err_handler)?;

							/*InvokeRule saplID*/
							recog.base.set_state(420);
							recog.saplID()?;

							recog.base.set_state(425);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							while _la==T__1 {
								{
								{
								recog.base.set_state(421);
								recog.base.match_token(T__1,&mut recog.err_handler)?;

								/*InvokeRule saplID*/
								recog.base.set_state(422);
								recog.saplID()?;

								}
								}
								recog.base.set_state(427);
								recog.err_handler.sync(&mut recog.base)?;
								_la = recog.base.input.la(1);
							}
							recog.base.set_state(429);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==T__43 {
								{
								/*InvokeRule arguments*/
								recog.base.set_state(428);
								recog.arguments()?;

								}
							}

							recog.base.set_state(431);
							recog.base.match_token(T__35,&mut recog.err_handler)?;

							}
						}

					 T__45 
						=> {
							{
							recog.base.set_state(433);
							recog.base.match_token(T__45,&mut recog.err_handler)?;

							/*InvokeRule saplID*/
							recog.base.set_state(434);
							recog.saplID()?;

							recog.base.set_state(439);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							while _la==T__1 {
								{
								{
								recog.base.set_state(435);
								recog.base.match_token(T__1,&mut recog.err_handler)?;

								/*InvokeRule saplID*/
								recog.base.set_state(436);
								recog.saplID()?;

								}
								}
								recog.base.set_state(441);
								recog.err_handler.sync(&mut recog.base)?;
								_la = recog.base.input.la(1);
							}
							recog.base.set_state(443);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==T__43 {
								{
								/*InvokeRule arguments*/
								recog.base.set_state(442);
								recog.arguments()?;

								}
							}

							recog.base.set_state(445);
							recog.base.match_token(T__35,&mut recog.err_handler)?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					}
				}

			 T__47 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(449);
					recog.base.match_token(T__47,&mut recog.err_handler)?;

					/*InvokeRule subscript*/
					recog.base.set_state(450);
					recog.subscript()?;

					recog.base.set_state(451);
					recog.base.match_token(T__48,&mut recog.err_handler)?;

					}
				}

			 T__49 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(453);
					recog.base.match_token(T__49,&mut recog.err_handler)?;

					recog.base.set_state(470);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(53,&mut recog.base)? {
						1 =>{
							{
							recog.base.set_state(458);
							recog.err_handler.sync(&mut recog.base)?;
							match recog.base.input.la(1) {
							 T__59 | T__60 | T__61 | T__62 | ID 
								=> {
									{
									/*InvokeRule saplID*/
									recog.base.set_state(454);
									recog.saplID()?;

									}
								}

							 T__47 
								=> {
									{
									recog.base.set_state(455);
									recog.base.match_token(T__47,&mut recog.err_handler)?;

									recog.base.set_state(456);
									recog.base.match_token(STRING,&mut recog.err_handler)?;

									recog.base.set_state(457);
									recog.base.match_token(T__48,&mut recog.err_handler)?;

									}
								}

								_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
							}
							}
						}
					,
						2 =>{
							{
							recog.base.set_state(464);
							recog.err_handler.sync(&mut recog.base)?;
							match recog.base.input.la(1) {
							 T__2 
								=> {
									{
									recog.base.set_state(460);
									recog.base.match_token(T__2,&mut recog.err_handler)?;

									}
								}

							 T__47 
								=> {
									{
									recog.base.set_state(461);
									recog.base.match_token(T__47,&mut recog.err_handler)?;

									recog.base.set_state(462);
									recog.base.match_token(T__2,&mut recog.err_handler)?;

									recog.base.set_state(463);
									recog.base.match_token(T__48,&mut recog.err_handler)?;

									}
								}

								_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
							}
							}
						}
					,
						3 =>{
							{
							recog.base.set_state(466);
							recog.base.match_token(T__47,&mut recog.err_handler)?;

							/*InvokeRule signedNumber*/
							recog.base.set_state(467);
							recog.signedNumber()?;

							recog.base.set_state(468);
							recog.base.match_token(T__48,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- subscript ----------------
pub type SubscriptContextAll<'input> = SubscriptContext<'input>;


pub type SubscriptContext<'input> = BaseParserRuleContext<'input,SubscriptContextExt<'input>>;

#[derive(Clone)]
pub struct SubscriptContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for SubscriptContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for SubscriptContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_subscript(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_subscript(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for SubscriptContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_subscript(self);
	}
}

impl<'input> CustomRuleContext<'input> for SubscriptContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_subscript }
	//fn type_rule_index() -> usize where Self: Sized { RULE_subscript }
}
antlr_rust::tid!{SubscriptContextExt<'a>}

impl<'input> SubscriptContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SubscriptContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SubscriptContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SubscriptContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<SubscriptContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token STRING in current rule
fn STRING_all(&self) -> Vec<Rc<TerminalNode<'input,SAPLParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token STRING, starting from 0.
/// Returns `None` if number of children corresponding to token STRING is less or equal than `i`.
fn STRING(&self, i: usize) -> Option<Rc<TerminalNode<'input,SAPLParserContextType>>> where Self:Sized{
	self.get_token(STRING, i)
}
fn signedNumber_all(&self) ->  Vec<Rc<SignedNumberContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn signedNumber(&self, i: usize) -> Option<Rc<SignedNumberContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SubscriptContextAttrs<'input> for SubscriptContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn subscript(&mut self,)
	-> Result<Rc<SubscriptContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SubscriptContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_subscript);
        let mut _localctx: Rc<SubscriptContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(513);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(60,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(474);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(475);
					recog.base.match_token(T__2,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule signedNumber*/
					recog.base.set_state(476);
					recog.signedNumber()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(478);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__39 || _la==JSONNUMBER {
						{
						/*InvokeRule signedNumber*/
						recog.base.set_state(477);
						recog.signedNumber()?;

						}
					}

					recog.base.set_state(480);
					recog.base.match_token(T__50,&mut recog.err_handler)?;

					recog.base.set_state(482);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__39 || _la==JSONNUMBER {
						{
						/*InvokeRule signedNumber*/
						recog.base.set_state(481);
						recog.signedNumber()?;

						}
					}

					recog.base.set_state(486);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__50 {
						{
						recog.base.set_state(484);
						recog.base.match_token(T__50,&mut recog.err_handler)?;

						/*InvokeRule signedNumber*/
						recog.base.set_state(485);
						recog.signedNumber()?;

						}
					}

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(488);
					recog.base.match_token(T__43,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(489);
					recog.expression()?;

					recog.base.set_state(490);
					recog.base.match_token(T__44,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					recog.base.set_state(492);
					recog.base.match_token(T__51,&mut recog.err_handler)?;

					recog.base.set_state(493);
					recog.base.match_token(T__43,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(494);
					recog.expression()?;

					recog.base.set_state(495);
					recog.base.match_token(T__44,&mut recog.err_handler)?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule signedNumber*/
					recog.base.set_state(497);
					recog.signedNumber()?;

					recog.base.set_state(502);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__24 {
						{
						{
						recog.base.set_state(498);
						recog.base.match_token(T__24,&mut recog.err_handler)?;

						/*InvokeRule signedNumber*/
						recog.base.set_state(499);
						recog.signedNumber()?;

						}
						}
						recog.base.set_state(504);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					recog.base.set_state(505);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					recog.base.set_state(510);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__24 {
						{
						{
						recog.base.set_state(506);
						recog.base.match_token(T__24,&mut recog.err_handler)?;

						recog.base.set_state(507);
						recog.base.match_token(STRING,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(512);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- value ----------------
pub type ValueContextAll<'input> = ValueContext<'input>;


pub type ValueContext<'input> = BaseParserRuleContext<'input,ValueContextExt<'input>>;

#[derive(Clone)]
pub struct ValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for ValueContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for ValueContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_value(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_value(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for ValueContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_value(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_value }
}
antlr_rust::tid!{ValueContextExt<'a>}

impl<'input> ValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ValueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ValueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ValueContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<ValueContextExt<'input>>{

fn object(&self) -> Option<Rc<ObjectContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn array(&self) -> Option<Rc<ArrayContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn numberLiteral(&self) -> Option<Rc<NumberLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn stringLiteral(&self) -> Option<Rc<StringLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn booleanLiteral(&self) -> Option<Rc<BooleanLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn nullLiteral(&self) -> Option<Rc<NullLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn undefinedLiteral(&self) -> Option<Rc<UndefinedLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ValueContextAttrs<'input> for ValueContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn value(&mut self,)
	-> Result<Rc<ValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_value);
        let mut _localctx: Rc<ValueContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(522);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__52 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule object*/
					recog.base.set_state(515);
					recog.object()?;

					}
				}

			 T__47 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule array*/
					recog.base.set_state(516);
					recog.array()?;

					}
				}

			 JSONNUMBER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule numberLiteral*/
					recog.base.set_state(517);
					recog.numberLiteral()?;

					}
				}

			 STRING 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule stringLiteral*/
					recog.base.set_state(518);
					recog.stringLiteral()?;

					}
				}

			 T__54 | T__55 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule booleanLiteral*/
					recog.base.set_state(519);
					recog.booleanLiteral()?;

					}
				}

			 T__56 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule nullLiteral*/
					recog.base.set_state(520);
					recog.nullLiteral()?;

					}
				}

			 T__57 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule undefinedLiteral*/
					recog.base.set_state(521);
					recog.undefinedLiteral()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- object ----------------
pub type ObjectContextAll<'input> = ObjectContext<'input>;


pub type ObjectContext<'input> = BaseParserRuleContext<'input,ObjectContextExt<'input>>;

#[derive(Clone)]
pub struct ObjectContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for ObjectContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for ObjectContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_object(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_object(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for ObjectContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_object(self);
	}
}

impl<'input> CustomRuleContext<'input> for ObjectContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_object }
	//fn type_rule_index() -> usize where Self: Sized { RULE_object }
}
antlr_rust::tid!{ObjectContextExt<'a>}

impl<'input> ObjectContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ObjectContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ObjectContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ObjectContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<ObjectContextExt<'input>>{

fn pair_all(&self) ->  Vec<Rc<PairContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn pair(&self, i: usize) -> Option<Rc<PairContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ObjectContextAttrs<'input> for ObjectContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn object(&mut self,)
	-> Result<Rc<ObjectContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ObjectContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_object);
        let mut _localctx: Rc<ObjectContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(524);
			recog.base.match_token(T__52,&mut recog.err_handler)?;

			recog.base.set_state(533);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==STRING {
				{
				/*InvokeRule pair*/
				recog.base.set_state(525);
				recog.pair()?;

				recog.base.set_state(530);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==T__24 {
					{
					{
					recog.base.set_state(526);
					recog.base.match_token(T__24,&mut recog.err_handler)?;

					/*InvokeRule pair*/
					recog.base.set_state(527);
					recog.pair()?;

					}
					}
					recog.base.set_state(532);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(535);
			recog.base.match_token(T__53,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- pair ----------------
pub type PairContextAll<'input> = PairContext<'input>;


pub type PairContext<'input> = BaseParserRuleContext<'input,PairContextExt<'input>>;

#[derive(Clone)]
pub struct PairContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for PairContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for PairContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_pair(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_pair(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for PairContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_pair(self);
	}
}

impl<'input> CustomRuleContext<'input> for PairContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pair }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pair }
}
antlr_rust::tid!{PairContextExt<'a>}

impl<'input> PairContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PairContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PairContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PairContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<PairContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,SAPLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PairContextAttrs<'input> for PairContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn pair(&mut self,)
	-> Result<Rc<PairContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PairContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_pair);
        let mut _localctx: Rc<PairContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(537);
			recog.base.match_token(STRING,&mut recog.err_handler)?;

			recog.base.set_state(538);
			recog.base.match_token(T__50,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(539);
			recog.expression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- array ----------------
pub type ArrayContextAll<'input> = ArrayContext<'input>;


pub type ArrayContext<'input> = BaseParserRuleContext<'input,ArrayContextExt<'input>>;

#[derive(Clone)]
pub struct ArrayContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for ArrayContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for ArrayContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_array(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_array(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for ArrayContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_array(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArrayContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_array }
	//fn type_rule_index() -> usize where Self: Sized { RULE_array }
}
antlr_rust::tid!{ArrayContextExt<'a>}

impl<'input> ArrayContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ArrayContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArrayContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ArrayContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<ArrayContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ArrayContextAttrs<'input> for ArrayContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn array(&mut self,)
	-> Result<Rc<ArrayContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArrayContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_array);
        let mut _localctx: Rc<ArrayContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(541);
			recog.base.match_token(T__47,&mut recog.err_handler)?;

			recog.base.set_state(550);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 34)) & !0x3f) == 0 && ((1usize << (_la - 34)) & ((1usize << (T__33 - 34)) | (1usize << (T__38 - 34)) | (1usize << (T__39 - 34)) | (1usize << (T__42 - 34)) | (1usize << (T__43 - 34)) | (1usize << (T__45 - 34)) | (1usize << (T__46 - 34)) | (1usize << (T__47 - 34)) | (1usize << (T__52 - 34)) | (1usize << (T__54 - 34)) | (1usize << (T__55 - 34)) | (1usize << (T__56 - 34)) | (1usize << (T__57 - 34)) | (1usize << (T__59 - 34)) | (1usize << (T__60 - 34)) | (1usize << (T__61 - 34)) | (1usize << (T__62 - 34)) | (1usize << (ID - 34)))) != 0) || _la==JSONNUMBER || _la==STRING {
				{
				/*InvokeRule expression*/
				recog.base.set_state(542);
				recog.expression()?;

				recog.base.set_state(547);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==T__24 {
					{
					{
					recog.base.set_state(543);
					recog.base.match_token(T__24,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(544);
					recog.expression()?;

					}
					}
					recog.base.set_state(549);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(552);
			recog.base.match_token(T__48,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- booleanLiteral ----------------
pub type BooleanLiteralContextAll<'input> = BooleanLiteralContext<'input>;


pub type BooleanLiteralContext<'input> = BaseParserRuleContext<'input,BooleanLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct BooleanLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for BooleanLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for BooleanLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_booleanLiteral(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_booleanLiteral(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for BooleanLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_booleanLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for BooleanLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_booleanLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_booleanLiteral }
}
antlr_rust::tid!{BooleanLiteralContextExt<'a>}

impl<'input> BooleanLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BooleanLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BooleanLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BooleanLiteralContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<BooleanLiteralContextExt<'input>>{


}

impl<'input> BooleanLiteralContextAttrs<'input> for BooleanLiteralContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn booleanLiteral(&mut self,)
	-> Result<Rc<BooleanLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BooleanLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_booleanLiteral);
        let mut _localctx: Rc<BooleanLiteralContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(554);
			_la = recog.base.input.la(1);
			if { !(_la==T__54 || _la==T__55) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- nullLiteral ----------------
pub type NullLiteralContextAll<'input> = NullLiteralContext<'input>;


pub type NullLiteralContext<'input> = BaseParserRuleContext<'input,NullLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct NullLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for NullLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for NullLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_nullLiteral(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_nullLiteral(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for NullLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_nullLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for NullLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nullLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nullLiteral }
}
antlr_rust::tid!{NullLiteralContextExt<'a>}

impl<'input> NullLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NullLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NullLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NullLiteralContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<NullLiteralContextExt<'input>>{


}

impl<'input> NullLiteralContextAttrs<'input> for NullLiteralContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn nullLiteral(&mut self,)
	-> Result<Rc<NullLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NullLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_nullLiteral);
        let mut _localctx: Rc<NullLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(556);
			recog.base.match_token(T__56,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- undefinedLiteral ----------------
pub type UndefinedLiteralContextAll<'input> = UndefinedLiteralContext<'input>;


pub type UndefinedLiteralContext<'input> = BaseParserRuleContext<'input,UndefinedLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct UndefinedLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for UndefinedLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for UndefinedLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_undefinedLiteral(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_undefinedLiteral(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for UndefinedLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_undefinedLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for UndefinedLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_undefinedLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_undefinedLiteral }
}
antlr_rust::tid!{UndefinedLiteralContextExt<'a>}

impl<'input> UndefinedLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<UndefinedLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UndefinedLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait UndefinedLiteralContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<UndefinedLiteralContextExt<'input>>{


}

impl<'input> UndefinedLiteralContextAttrs<'input> for UndefinedLiteralContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn undefinedLiteral(&mut self,)
	-> Result<Rc<UndefinedLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UndefinedLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_undefinedLiteral);
        let mut _localctx: Rc<UndefinedLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(558);
			recog.base.match_token(T__57,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- stringLiteral ----------------
pub type StringLiteralContextAll<'input> = StringLiteralContext<'input>;


pub type StringLiteralContext<'input> = BaseParserRuleContext<'input,StringLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct StringLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for StringLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for StringLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stringLiteral(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_stringLiteral(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for StringLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_stringLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for StringLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stringLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stringLiteral }
}
antlr_rust::tid!{StringLiteralContextExt<'a>}

impl<'input> StringLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StringLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StringLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StringLiteralContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<StringLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,SAPLParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}

}

impl<'input> StringLiteralContextAttrs<'input> for StringLiteralContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stringLiteral(&mut self,)
	-> Result<Rc<StringLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StringLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_stringLiteral);
        let mut _localctx: Rc<StringLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(560);
			recog.base.match_token(STRING,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- numberLiteral ----------------
pub type NumberLiteralContextAll<'input> = NumberLiteralContext<'input>;


pub type NumberLiteralContext<'input> = BaseParserRuleContext<'input,NumberLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct NumberLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for NumberLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for NumberLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_numberLiteral(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_numberLiteral(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for NumberLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_numberLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for NumberLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_numberLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_numberLiteral }
}
antlr_rust::tid!{NumberLiteralContextExt<'a>}

impl<'input> NumberLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NumberLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NumberLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NumberLiteralContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<NumberLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token JSONNUMBER
/// Returns `None` if there is no child corresponding to token JSONNUMBER
fn JSONNUMBER(&self) -> Option<Rc<TerminalNode<'input,SAPLParserContextType>>> where Self:Sized{
	self.get_token(JSONNUMBER, 0)
}

}

impl<'input> NumberLiteralContextAttrs<'input> for NumberLiteralContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn numberLiteral(&mut self,)
	-> Result<Rc<NumberLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NumberLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_numberLiteral);
        let mut _localctx: Rc<NumberLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(562);
			recog.base.match_token(JSONNUMBER,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- filterComponent ----------------
pub type FilterComponentContextAll<'input> = FilterComponentContext<'input>;


pub type FilterComponentContext<'input> = BaseParserRuleContext<'input,FilterComponentContextExt<'input>>;

#[derive(Clone)]
pub struct FilterComponentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for FilterComponentContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for FilterComponentContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_filterComponent(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_filterComponent(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for FilterComponentContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_filterComponent(self);
	}
}

impl<'input> CustomRuleContext<'input> for FilterComponentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_filterComponent }
	//fn type_rule_index() -> usize where Self: Sized { RULE_filterComponent }
}
antlr_rust::tid!{FilterComponentContextExt<'a>}

impl<'input> FilterComponentContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FilterComponentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FilterComponentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FilterComponentContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<FilterComponentContextExt<'input>>{

fn saplID_all(&self) ->  Vec<Rc<SaplIDContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn saplID(&self, i: usize) -> Option<Rc<SaplIDContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn arguments(&self) -> Option<Rc<ArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn filterStatement_all(&self) ->  Vec<Rc<FilterStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn filterStatement(&self, i: usize) -> Option<Rc<FilterStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> FilterComponentContextAttrs<'input> for FilterComponentContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn filterComponent(&mut self,)
	-> Result<Rc<FilterComponentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FilterComponentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_filterComponent);
        let mut _localctx: Rc<FilterComponentContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(589);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__58 | T__59 | T__60 | T__61 | T__62 | ID 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(565);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__58 {
						{
						recog.base.set_state(564);
						recog.base.match_token(T__58,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule saplID*/
					recog.base.set_state(567);
					recog.saplID()?;

					recog.base.set_state(572);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__1 {
						{
						{
						recog.base.set_state(568);
						recog.base.match_token(T__1,&mut recog.err_handler)?;

						/*InvokeRule saplID*/
						recog.base.set_state(569);
						recog.saplID()?;

						}
						}
						recog.base.set_state(574);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(576);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__43 {
						{
						/*InvokeRule arguments*/
						recog.base.set_state(575);
						recog.arguments()?;

						}
					}

					}
				}

			 T__52 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(578);
					recog.base.match_token(T__52,&mut recog.err_handler)?;

					/*InvokeRule filterStatement*/
					recog.base.set_state(579);
					recog.filterStatement()?;

					recog.base.set_state(584);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__24 {
						{
						{
						recog.base.set_state(580);
						recog.base.match_token(T__24,&mut recog.err_handler)?;

						/*InvokeRule filterStatement*/
						recog.base.set_state(581);
						recog.filterStatement()?;

						}
						}
						recog.base.set_state(586);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(587);
					recog.base.match_token(T__53,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- filterStatement ----------------
pub type FilterStatementContextAll<'input> = FilterStatementContext<'input>;


pub type FilterStatementContext<'input> = BaseParserRuleContext<'input,FilterStatementContextExt<'input>>;

#[derive(Clone)]
pub struct FilterStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for FilterStatementContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for FilterStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_filterStatement(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_filterStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for FilterStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_filterStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for FilterStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_filterStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_filterStatement }
}
antlr_rust::tid!{FilterStatementContextExt<'a>}

impl<'input> FilterStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FilterStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FilterStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FilterStatementContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<FilterStatementContextExt<'input>>{

fn saplID_all(&self) ->  Vec<Rc<SaplIDContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn saplID(&self, i: usize) -> Option<Rc<SaplIDContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn arguments(&self) -> Option<Rc<ArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FilterStatementContextAttrs<'input> for FilterStatementContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn filterStatement(&mut self,)
	-> Result<Rc<FilterStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FilterStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_filterStatement);
        let mut _localctx: Rc<FilterStatementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(592);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__58 {
				{
				recog.base.set_state(591);
				recog.base.match_token(T__58,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(595);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__46 {
				{
				recog.base.set_state(594);
				recog.base.match_token(T__46,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(597);
			recog.base.match_token(T__50,&mut recog.err_handler)?;

			/*InvokeRule saplID*/
			recog.base.set_state(598);
			recog.saplID()?;

			recog.base.set_state(603);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__1 {
				{
				{
				recog.base.set_state(599);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

				/*InvokeRule saplID*/
				recog.base.set_state(600);
				recog.saplID()?;

				}
				}
				recog.base.set_state(605);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(607);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__43 {
				{
				/*InvokeRule arguments*/
				recog.base.set_state(606);
				recog.arguments()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- signedNumber ----------------
pub type SignedNumberContextAll<'input> = SignedNumberContext<'input>;


pub type SignedNumberContext<'input> = BaseParserRuleContext<'input,SignedNumberContextExt<'input>>;

#[derive(Clone)]
pub struct SignedNumberContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for SignedNumberContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for SignedNumberContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_signedNumber(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_signedNumber(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for SignedNumberContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_signedNumber(self);
	}
}

impl<'input> CustomRuleContext<'input> for SignedNumberContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_signedNumber }
	//fn type_rule_index() -> usize where Self: Sized { RULE_signedNumber }
}
antlr_rust::tid!{SignedNumberContextExt<'a>}

impl<'input> SignedNumberContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SignedNumberContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SignedNumberContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SignedNumberContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<SignedNumberContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token JSONNUMBER
/// Returns `None` if there is no child corresponding to token JSONNUMBER
fn JSONNUMBER(&self) -> Option<Rc<TerminalNode<'input,SAPLParserContextType>>> where Self:Sized{
	self.get_token(JSONNUMBER, 0)
}

}

impl<'input> SignedNumberContextAttrs<'input> for SignedNumberContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn signedNumber(&mut self,)
	-> Result<Rc<SignedNumberContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SignedNumberContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_signedNumber);
        let mut _localctx: Rc<SignedNumberContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(610);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__39 {
				{
				recog.base.set_state(609);
				recog.base.match_token(T__39,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(612);
			recog.base.match_token(JSONNUMBER,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- saplID ----------------
pub type SaplIDContextAll<'input> = SaplIDContext<'input>;


pub type SaplIDContext<'input> = BaseParserRuleContext<'input,SaplIDContextExt<'input>>;

#[derive(Clone)]
pub struct SaplIDContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for SaplIDContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for SaplIDContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_saplID(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_saplID(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for SaplIDContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_saplID(self);
	}
}

impl<'input> CustomRuleContext<'input> for SaplIDContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_saplID }
	//fn type_rule_index() -> usize where Self: Sized { RULE_saplID }
}
antlr_rust::tid!{SaplIDContextExt<'a>}

impl<'input> SaplIDContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SaplIDContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SaplIDContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SaplIDContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<SaplIDContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ID
/// Returns `None` if there is no child corresponding to token ID
fn ID(&self) -> Option<Rc<TerminalNode<'input,SAPLParserContextType>>> where Self:Sized{
	self.get_token(ID, 0)
}
fn reservedID(&self) -> Option<Rc<ReservedIDContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SaplIDContextAttrs<'input> for SaplIDContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn saplID(&mut self,)
	-> Result<Rc<SaplIDContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SaplIDContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 84, RULE_saplID);
        let mut _localctx: Rc<SaplIDContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(616);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 ID 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(614);
					recog.base.match_token(ID,&mut recog.err_handler)?;

					}
				}

			 T__59 | T__60 | T__61 | T__62 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule reservedID*/
					recog.base.set_state(615);
					recog.reservedID()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- reservedID ----------------
pub type ReservedIDContextAll<'input> = ReservedIDContext<'input>;


pub type ReservedIDContext<'input> = BaseParserRuleContext<'input,ReservedIDContextExt<'input>>;

#[derive(Clone)]
pub struct ReservedIDContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SAPLParserContext<'input> for ReservedIDContext<'input>{}

impl<'input,'a> Listenable<dyn SAPLListener<'input> + 'a> for ReservedIDContext<'input>{
		fn enter(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_reservedID(self);
		}
		fn exit(&self,listener: &mut (dyn SAPLListener<'input> + 'a)) {
			listener.exit_reservedID(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn SAPLVisitor<'input> + 'a> for ReservedIDContext<'input>{
	fn accept(&self,visitor: &mut (dyn SAPLVisitor<'input> + 'a)) {
		visitor.visit_reservedID(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReservedIDContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SAPLParserContextType;
	fn get_rule_index(&self) -> usize { RULE_reservedID }
	//fn type_rule_index() -> usize where Self: Sized { RULE_reservedID }
}
antlr_rust::tid!{ReservedIDContextExt<'a>}

impl<'input> ReservedIDContextExt<'input>{
	fn new(parent: Option<Rc<dyn SAPLParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReservedIDContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReservedIDContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReservedIDContextAttrs<'input>: SAPLParserContext<'input> + BorrowMut<ReservedIDContextExt<'input>>{


}

impl<'input> ReservedIDContextAttrs<'input> for ReservedIDContext<'input>{}

impl<'input, I, H> SAPLParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn reservedID(&mut self,)
	-> Result<Rc<ReservedIDContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReservedIDContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 86, RULE_reservedID);
        let mut _localctx: Rc<ReservedIDContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(618);
			_la = recog.base.input.la(1);
			if { !(((((_la - 60)) & !0x3f) == 0 && ((1usize << (_la - 60)) & ((1usize << (T__59 - 60)) | (1usize << (T__60 - 60)) | (1usize << (T__61 - 60)) | (1usize << (T__62 - 60)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x49\u{26f}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x04\
	\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x04\x29\x09\
	\x29\x04\x2a\x09\x2a\x04\x2b\x09\x2b\x04\x2c\x09\x2c\x04\x2d\x09\x2d\x03\
	\x02\x07\x02\x5c\x0a\x02\x0c\x02\x0e\x02\x5f\x0b\x02\x03\x02\x07\x02\x62\
	\x0a\x02\x0c\x02\x0e\x02\x65\x0b\x02\x03\x02\x03\x02\x03\x02\x03\x03\x03\
	\x03\x03\x03\x03\x03\x05\x03\x6e\x0a\x03\x03\x04\x03\x04\x03\x04\x07\x04\
	\x73\x0a\x04\x0c\x04\x0e\x04\x76\x0b\x04\x03\x04\x03\x04\x03\x04\x03\x05\
	\x03\x05\x03\x05\x07\x05\x7e\x0a\x05\x0c\x05\x0e\x05\u{81}\x0b\x05\x03\x05\
	\x03\x05\x03\x05\x03\x06\x03\x06\x03\x06\x07\x06\u{89}\x0a\x06\x0c\x06\x0e\
	\x06\u{8c}\x0b\x06\x03\x06\x03\x06\x03\x06\x03\x07\x03\x07\x05\x07\u{93}\
	\x0a\x07\x03\x07\x03\x07\x03\x07\x03\x08\x03\x08\x05\x08\u{9a}\x0a\x08\x03\
	\x09\x03\x09\x03\x09\x03\x09\x03\x09\x05\x09\u{a1}\x0a\x09\x03\x09\x03\x09\
	\x03\x09\x07\x09\u{a6}\x0a\x09\x0c\x09\x0e\x09\u{a9}\x0b\x09\x03\x09\x06\
	\x09\u{ac}\x0a\x09\x0d\x09\x0e\x09\u{ad}\x03\x0a\x03\x0a\x03\x0b\x03\x0b\
	\x03\x0b\x03\x0b\x05\x0b\u{b6}\x0a\x0b\x03\x0b\x03\x0b\x03\x0b\x07\x0b\u{bb}\
	\x0a\x0b\x0c\x0b\x0e\x0b\u{be}\x0b\x0b\x05\x0b\u{c0}\x0a\x0b\x03\x0b\x03\
	\x0b\x07\x0b\u{c4}\x0a\x0b\x0c\x0b\x0e\x0b\u{c7}\x0b\x0b\x03\x0b\x03\x0b\
	\x07\x0b\u{cb}\x0a\x0b\x0c\x0b\x0e\x0b\u{ce}\x0b\x0b\x03\x0b\x03\x0b\x07\
	\x0b\u{d2}\x0a\x0b\x0c\x0b\x0e\x0b\u{d5}\x0b\x0b\x03\x0c\x03\x0c\x03\x0d\
	\x03\x0d\x03\x0e\x03\x0e\x05\x0e\u{dd}\x0a\x0e\x03\x0e\x03\x0e\x03\x0f\x03\
	\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x07\x0f\u{e9}\x0a\x0f\
	\x0c\x0f\x0e\x0f\u{ec}\x0b\x0f\x05\x0f\u{ee}\x0a\x0f\x03\x10\x03\x10\x03\
	\x11\x03\x11\x03\x11\x07\x11\u{f5}\x0a\x11\x0c\x11\x0e\x11\u{f8}\x0b\x11\
	\x03\x12\x03\x12\x03\x12\x07\x12\u{fd}\x0a\x12\x0c\x12\x0e\x12\u{100}\x0b\
	\x12\x03\x13\x03\x13\x03\x13\x07\x13\u{105}\x0a\x13\x0c\x13\x0e\x13\u{108}\
	\x0b\x13\x03\x14\x03\x14\x03\x14\x07\x14\u{10d}\x0a\x14\x0c\x14\x0e\x14\
	\u{110}\x0b\x14\x03\x15\x03\x15\x03\x15\x07\x15\u{115}\x0a\x15\x0c\x15\x0e\
	\x15\u{118}\x0b\x15\x03\x16\x03\x16\x03\x16\x05\x16\u{11d}\x0a\x16\x03\x17\
	\x03\x17\x03\x17\x05\x17\u{122}\x0a\x17\x03\x18\x03\x18\x03\x18\x07\x18\
	\u{127}\x0a\x18\x0c\x18\x0e\x18\u{12a}\x0b\x18\x03\x19\x03\x19\x03\x19\x07\
	\x19\u{12f}\x0a\x19\x0c\x19\x0e\x19\u{132}\x0b\x19\x03\x1a\x03\x1a\x03\x1a\
	\x05\x1a\u{137}\x0a\x1a\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x05\x1b\
	\u{13e}\x0a\x1b\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x07\x1c\u{144}\x0a\x1c\x0c\
	\x1c\x0e\x1c\u{147}\x0b\x1c\x03\x1c\x03\x1c\x07\x1c\u{14b}\x0a\x1c\x0c\x1c\
	\x0e\x1c\u{14e}\x0b\x1c\x03\x1c\x03\x1c\x03\x1c\x07\x1c\u{153}\x0a\x1c\x0c\
	\x1c\x0e\x1c\u{156}\x0b\x1c\x03\x1c\x03\x1c\x07\x1c\u{15a}\x0a\x1c\x0c\x1c\
	\x0e\x1c\u{15d}\x0b\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x07\x1c\u{163}\x0a\
	\x1c\x0c\x1c\x0e\x1c\u{166}\x0b\x1c\x03\x1c\x05\x1c\u{169}\x0a\x1c\x03\x1c\
	\x03\x1c\x07\x1c\u{16d}\x0a\x1c\x0c\x1c\x0e\x1c\u{170}\x0b\x1c\x03\x1c\x03\
	\x1c\x03\x1c\x03\x1c\x07\x1c\u{176}\x0a\x1c\x0c\x1c\x0e\x1c\u{179}\x0b\x1c\
	\x03\x1c\x05\x1c\u{17c}\x0a\x1c\x03\x1c\x03\x1c\x07\x1c\u{180}\x0a\x1c\x0c\
	\x1c\x0e\x1c\u{183}\x0b\x1c\x03\x1c\x03\x1c\x07\x1c\u{187}\x0a\x1c\x0c\x1c\
	\x0e\x1c\u{18a}\x0b\x1c\x03\x1c\x03\x1c\x07\x1c\u{18e}\x0a\x1c\x0c\x1c\x0e\
	\x1c\u{191}\x0b\x1c\x05\x1c\u{193}\x0a\x1c\x03\x1d\x03\x1d\x03\x1d\x03\x1d\
	\x07\x1d\u{199}\x0a\x1d\x0c\x1d\x0e\x1d\u{19c}\x0b\x1d\x05\x1d\u{19e}\x0a\
	\x1d\x03\x1d\x03\x1d\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\
	\x1e\x03\x1e\x07\x1e\u{1aa}\x0a\x1e\x0c\x1e\x0e\x1e\u{1ad}\x0b\x1e\x03\x1e\
	\x05\x1e\u{1b0}\x0a\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\
	\x07\x1e\u{1b8}\x0a\x1e\x0c\x1e\x0e\x1e\u{1bb}\x0b\x1e\x03\x1e\x05\x1e\u{1be}\
	\x0a\x1e\x03\x1e\x03\x1e\x05\x1e\u{1c2}\x0a\x1e\x03\x1e\x03\x1e\x03\x1e\
	\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x05\x1e\u{1cd}\x0a\x1e\
	\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x05\x1e\u{1d3}\x0a\x1e\x03\x1e\x03\x1e\
	\x03\x1e\x03\x1e\x05\x1e\u{1d9}\x0a\x1e\x05\x1e\u{1db}\x0a\x1e\x03\x1f\x03\
	\x1f\x03\x1f\x03\x1f\x05\x1f\u{1e1}\x0a\x1f\x03\x1f\x03\x1f\x05\x1f\u{1e5}\
	\x0a\x1f\x03\x1f\x03\x1f\x05\x1f\u{1e9}\x0a\x1f\x03\x1f\x03\x1f\x03\x1f\
	\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\
	\x07\x1f\u{1f7}\x0a\x1f\x0c\x1f\x0e\x1f\u{1fa}\x0b\x1f\x03\x1f\x03\x1f\x03\
	\x1f\x07\x1f\u{1ff}\x0a\x1f\x0c\x1f\x0e\x1f\u{202}\x0b\x1f\x05\x1f\u{204}\
	\x0a\x1f\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x05\x20\
	\u{20d}\x0a\x20\x03\x21\x03\x21\x03\x21\x03\x21\x07\x21\u{213}\x0a\x21\x0c\
	\x21\x0e\x21\u{216}\x0b\x21\x05\x21\u{218}\x0a\x21\x03\x21\x03\x21\x03\x22\
	\x03\x22\x03\x22\x03\x22\x03\x23\x03\x23\x03\x23\x03\x23\x07\x23\u{224}\
	\x0a\x23\x0c\x23\x0e\x23\u{227}\x0b\x23\x05\x23\u{229}\x0a\x23\x03\x23\x03\
	\x23\x03\x24\x03\x24\x03\x25\x03\x25\x03\x26\x03\x26\x03\x27\x03\x27\x03\
	\x28\x03\x28\x03\x29\x05\x29\u{238}\x0a\x29\x03\x29\x03\x29\x03\x29\x07\
	\x29\u{23d}\x0a\x29\x0c\x29\x0e\x29\u{240}\x0b\x29\x03\x29\x05\x29\u{243}\
	\x0a\x29\x03\x29\x03\x29\x03\x29\x03\x29\x07\x29\u{249}\x0a\x29\x0c\x29\
	\x0e\x29\u{24c}\x0b\x29\x03\x29\x03\x29\x05\x29\u{250}\x0a\x29\x03\x2a\x05\
	\x2a\u{253}\x0a\x2a\x03\x2a\x05\x2a\u{256}\x0a\x2a\x03\x2a\x03\x2a\x03\x2a\
	\x03\x2a\x07\x2a\u{25c}\x0a\x2a\x0c\x2a\x0e\x2a\u{25f}\x0b\x2a\x03\x2a\x05\
	\x2a\u{262}\x0a\x2a\x03\x2b\x05\x2b\u{265}\x0a\x2b\x03\x2b\x03\x2b\x03\x2c\
	\x03\x2c\x05\x2c\u{26b}\x0a\x2c\x03\x2d\x03\x2d\x03\x2d\x02\x02\x2e\x02\
	\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\
	\x28\x2a\x2c\x2e\x30\x32\x34\x36\x38\x3a\x3c\x3e\x40\x42\x44\x46\x48\x4a\
	\x4c\x4e\x50\x52\x54\x56\x58\x02\x0b\x03\x02\x0c\x11\x03\x02\x17\x18\x03\
	\x02\x21\x23\x03\x02\x24\x28\x03\x02\x29\x2a\x04\x02\x05\x05\x2b\x2c\x04\
	\x02\x29\x2a\x2d\x2d\x03\x02\x39\x3a\x03\x02\x3e\x41\x02\u{2a6}\x02\x5d\
	\x03\x02\x02\x02\x04\x69\x03\x02\x02\x02\x06\x6f\x03\x02\x02\x02\x08\x7a\
	\x03\x02\x02\x02\x0a\u{85}\x03\x02\x02\x02\x0c\u{90}\x03\x02\x02\x02\x0e\
	\u{99}\x03\x02\x02\x02\x10\u{9b}\x03\x02\x02\x02\x12\u{af}\x03\x02\x02\x02\
	\x14\u{b1}\x03\x02\x02\x02\x16\u{d6}\x03\x02\x02\x02\x18\u{d8}\x03\x02\x02\
	\x02\x1a\u{dc}\x03\x02\x02\x02\x1c\u{e0}\x03\x02\x02\x02\x1e\u{ef}\x03\x02\
	\x02\x02\x20\u{f1}\x03\x02\x02\x02\x22\u{f9}\x03\x02\x02\x02\x24\u{101}\
	\x03\x02\x02\x02\x26\u{109}\x03\x02\x02\x02\x28\u{111}\x03\x02\x02\x02\x2a\
	\u{119}\x03\x02\x02\x02\x2c\u{11e}\x03\x02\x02\x02\x2e\u{123}\x03\x02\x02\
	\x02\x30\u{12b}\x03\x02\x02\x02\x32\u{136}\x03\x02\x02\x02\x34\u{138}\x03\
	\x02\x02\x02\x36\u{192}\x03\x02\x02\x02\x38\u{194}\x03\x02\x02\x02\x3a\u{1da}\
	\x03\x02\x02\x02\x3c\u{203}\x03\x02\x02\x02\x3e\u{20c}\x03\x02\x02\x02\x40\
	\u{20e}\x03\x02\x02\x02\x42\u{21b}\x03\x02\x02\x02\x44\u{21f}\x03\x02\x02\
	\x02\x46\u{22c}\x03\x02\x02\x02\x48\u{22e}\x03\x02\x02\x02\x4a\u{230}\x03\
	\x02\x02\x02\x4c\u{232}\x03\x02\x02\x02\x4e\u{234}\x03\x02\x02\x02\x50\u{24f}\
	\x03\x02\x02\x02\x52\u{252}\x03\x02\x02\x02\x54\u{264}\x03\x02\x02\x02\x56\
	\u{26a}\x03\x02\x02\x02\x58\u{26c}\x03\x02\x02\x02\x5a\x5c\x05\x04\x03\x02\
	\x5b\x5a\x03\x02\x02\x02\x5c\x5f\x03\x02\x02\x02\x5d\x5b\x03\x02\x02\x02\
	\x5d\x5e\x03\x02\x02\x02\x5e\x63\x03\x02\x02\x02\x5f\x5d\x03\x02\x02\x02\
	\x60\x62\x05\x0c\x07\x02\x61\x60\x03\x02\x02\x02\x62\x65\x03\x02\x02\x02\
	\x63\x61\x03\x02\x02\x02\x63\x64\x03\x02\x02\x02\x64\x66\x03\x02\x02\x02\
	\x65\x63\x03\x02\x02\x02\x66\x67\x05\x0e\x08\x02\x67\x68\x07\x02\x02\x03\
	\x68\x03\x03\x02\x02\x02\x69\x6d\x07\x03\x02\x02\x6a\x6e\x05\x06\x04\x02\
	\x6b\x6e\x05\x08\x05\x02\x6c\x6e\x05\x0a\x06\x02\x6d\x6a\x03\x02\x02\x02\
	\x6d\x6b\x03\x02\x02\x02\x6d\x6c\x03\x02\x02\x02\x6e\x05\x03\x02\x02\x02\
	\x6f\x74\x05\x56\x2c\x02\x70\x71\x07\x04\x02\x02\x71\x73\x05\x56\x2c\x02\
	\x72\x70\x03\x02\x02\x02\x73\x76\x03\x02\x02\x02\x74\x72\x03\x02\x02\x02\
	\x74\x75\x03\x02\x02\x02\x75\x77\x03\x02\x02\x02\x76\x74\x03\x02\x02\x02\
	\x77\x78\x07\x04\x02\x02\x78\x79\x07\x05\x02\x02\x79\x07\x03\x02\x02\x02\
	\x7a\x7f\x05\x56\x2c\x02\x7b\x7c\x07\x04\x02\x02\x7c\x7e\x05\x56\x2c\x02\
	\x7d\x7b\x03\x02\x02\x02\x7e\u{81}\x03\x02\x02\x02\x7f\x7d\x03\x02\x02\x02\
	\x7f\u{80}\x03\x02\x02\x02\u{80}\u{82}\x03\x02\x02\x02\u{81}\x7f\x03\x02\
	\x02\x02\u{82}\u{83}\x07\x06\x02\x02\u{83}\u{84}\x05\x56\x2c\x02\u{84}\x09\
	\x03\x02\x02\x02\u{85}\u{8a}\x05\x56\x2c\x02\u{86}\u{87}\x07\x04\x02\x02\
	\u{87}\u{89}\x05\x56\x2c\x02\u{88}\u{86}\x03\x02\x02\x02\u{89}\u{8c}\x03\
	\x02\x02\x02\u{8a}\u{88}\x03\x02\x02\x02\u{8a}\u{8b}\x03\x02\x02\x02\u{8b}\
	\u{8d}\x03\x02\x02\x02\u{8c}\u{8a}\x03\x02\x02\x02\u{8d}\u{8e}\x07\x04\x02\
	\x02\u{8e}\u{8f}\x05\x56\x2c\x02\u{8f}\x0b\x03\x02\x02\x02\u{90}\u{92}\x05\
	\x58\x2d\x02\u{91}\u{93}\x07\x07\x02\x02\u{92}\u{91}\x03\x02\x02\x02\u{92}\
	\u{93}\x03\x02\x02\x02\u{93}\u{94}\x03\x02\x02\x02\u{94}\u{95}\x07\x08\x02\
	\x02\u{95}\u{96}\x05\x1e\x10\x02\u{96}\x0d\x03\x02\x02\x02\u{97}\u{9a}\x05\
	\x10\x09\x02\u{98}\u{9a}\x05\x14\x0b\x02\u{99}\u{97}\x03\x02\x02\x02\u{99}\
	\u{98}\x03\x02\x02\x02\u{9a}\x0f\x03\x02\x02\x02\u{9b}\u{9c}\x07\x09\x02\
	\x02\u{9c}\u{9d}\x07\x49\x02\x02\u{9d}\u{a0}\x05\x12\x0a\x02\u{9e}\u{9f}\
	\x07\x0a\x02\x02\u{9f}\u{a1}\x05\x18\x0d\x02\u{a0}\u{9e}\x03\x02\x02\x02\
	\u{a0}\u{a1}\x03\x02\x02\x02\u{a1}\u{a7}\x03\x02\x02\x02\u{a2}\u{a3}\x05\
	\x1c\x0f\x02\u{a3}\u{a4}\x07\x0b\x02\x02\u{a4}\u{a6}\x03\x02\x02\x02\u{a5}\
	\u{a2}\x03\x02\x02\x02\u{a6}\u{a9}\x03\x02\x02\x02\u{a7}\u{a5}\x03\x02\x02\
	\x02\u{a7}\u{a8}\x03\x02\x02\x02\u{a8}\u{ab}\x03\x02\x02\x02\u{a9}\u{a7}\
	\x03\x02\x02\x02\u{aa}\u{ac}\x05\x14\x0b\x02\u{ab}\u{aa}\x03\x02\x02\x02\
	\u{ac}\u{ad}\x03\x02\x02\x02\u{ad}\u{ab}\x03\x02\x02\x02\u{ad}\u{ae}\x03\
	\x02\x02\x02\u{ae}\x11\x03\x02\x02\x02\u{af}\u{b0}\x09\x02\x02\x02\u{b0}\
	\x13\x03\x02\x02\x02\u{b1}\u{b2}\x07\x12\x02\x02\u{b2}\u{b3}\x07\x49\x02\
	\x02\u{b3}\u{b5}\x05\x16\x0c\x02\u{b4}\u{b6}\x05\x18\x0d\x02\u{b5}\u{b4}\
	\x03\x02\x02\x02\u{b5}\u{b6}\x03\x02\x02\x02\u{b6}\u{bf}\x03\x02\x02\x02\
	\u{b7}\u{b8}\x07\x13\x02\x02\u{b8}\u{bc}\x05\x1a\x0e\x02\u{b9}\u{bb}\x05\
	\x1a\x0e\x02\u{ba}\u{b9}\x03\x02\x02\x02\u{bb}\u{be}\x03\x02\x02\x02\u{bc}\
	\u{ba}\x03\x02\x02\x02\u{bc}\u{bd}\x03\x02\x02\x02\u{bd}\u{c0}\x03\x02\x02\
	\x02\u{be}\u{bc}\x03\x02\x02\x02\u{bf}\u{b7}\x03\x02\x02\x02\u{bf}\u{c0}\
	\x03\x02\x02\x02\u{c0}\u{c5}\x03\x02\x02\x02\u{c1}\u{c2}\x07\x14\x02\x02\
	\u{c2}\u{c4}\x05\x1e\x10\x02\u{c3}\u{c1}\x03\x02\x02\x02\u{c4}\u{c7}\x03\
	\x02\x02\x02\u{c5}\u{c3}\x03\x02\x02\x02\u{c5}\u{c6}\x03\x02\x02\x02\u{c6}\
	\u{cc}\x03\x02\x02\x02\u{c7}\u{c5}\x03\x02\x02\x02\u{c8}\u{c9}\x07\x15\x02\
	\x02\u{c9}\u{cb}\x05\x1e\x10\x02\u{ca}\u{c8}\x03\x02\x02\x02\u{cb}\u{ce}\
	\x03\x02\x02\x02\u{cc}\u{ca}\x03\x02\x02\x02\u{cc}\u{cd}\x03\x02\x02\x02\
	\u{cd}\u{d3}\x03\x02\x02\x02\u{ce}\u{cc}\x03\x02\x02\x02\u{cf}\u{d0}\x07\
	\x16\x02\x02\u{d0}\u{d2}\x05\x1e\x10\x02\u{d1}\u{cf}\x03\x02\x02\x02\u{d2}\
	\u{d5}\x03\x02\x02\x02\u{d3}\u{d1}\x03\x02\x02\x02\u{d3}\u{d4}\x03\x02\x02\
	\x02\u{d4}\x15\x03\x02\x02\x02\u{d5}\u{d3}\x03\x02\x02\x02\u{d6}\u{d7}\x09\
	\x03\x02\x02\u{d7}\x17\x03\x02\x02\x02\u{d8}\u{d9}\x05\x1e\x10\x02\u{d9}\
	\x19\x03\x02\x02\x02\u{da}\u{dd}\x05\x1c\x0f\x02\u{db}\u{dd}\x05\x1e\x10\
	\x02\u{dc}\u{da}\x03\x02\x02\x02\u{dc}\u{db}\x03\x02\x02\x02\u{dd}\u{de}\
	\x03\x02\x02\x02\u{de}\u{df}\x07\x0b\x02\x02\u{df}\x1b\x03\x02\x02\x02\u{e0}\
	\u{e1}\x07\x19\x02\x02\u{e1}\u{e2}\x07\x42\x02\x02\u{e2}\u{e3}\x07\x1a\x02\
	\x02\u{e3}\u{ed}\x05\x1e\x10\x02\u{e4}\u{e5}\x07\x08\x02\x02\u{e5}\u{ea}\
	\x05\x1e\x10\x02\u{e6}\u{e7}\x07\x1b\x02\x02\u{e7}\u{e9}\x05\x1e\x10\x02\
	\u{e8}\u{e6}\x03\x02\x02\x02\u{e9}\u{ec}\x03\x02\x02\x02\u{ea}\u{e8}\x03\
	\x02\x02\x02\u{ea}\u{eb}\x03\x02\x02\x02\u{eb}\u{ee}\x03\x02\x02\x02\u{ec}\
	\u{ea}\x03\x02\x02\x02\u{ed}\u{e4}\x03\x02\x02\x02\u{ed}\u{ee}\x03\x02\x02\
	\x02\u{ee}\x1d\x03\x02\x02\x02\u{ef}\u{f0}\x05\x20\x11\x02\u{f0}\x1f\x03\
	\x02\x02\x02\u{f1}\u{f6}\x05\x22\x12\x02\u{f2}\u{f3}\x07\x1c\x02\x02\u{f3}\
	\u{f5}\x05\x22\x12\x02\u{f4}\u{f2}\x03\x02\x02\x02\u{f5}\u{f8}\x03\x02\x02\
	\x02\u{f6}\u{f4}\x03\x02\x02\x02\u{f6}\u{f7}\x03\x02\x02\x02\u{f7}\x21\x03\
	\x02\x02\x02\u{f8}\u{f6}\x03\x02\x02\x02\u{f9}\u{fe}\x05\x24\x13\x02\u{fa}\
	\u{fb}\x07\x1d\x02\x02\u{fb}\u{fd}\x05\x24\x13\x02\u{fc}\u{fa}\x03\x02\x02\
	\x02\u{fd}\u{100}\x03\x02\x02\x02\u{fe}\u{fc}\x03\x02\x02\x02\u{fe}\u{ff}\
	\x03\x02\x02\x02\u{ff}\x23\x03\x02\x02\x02\u{100}\u{fe}\x03\x02\x02\x02\
	\u{101}\u{106}\x05\x26\x14\x02\u{102}\u{103}\x07\x1e\x02\x02\u{103}\u{105}\
	\x05\x26\x14\x02\u{104}\u{102}\x03\x02\x02\x02\u{105}\u{108}\x03\x02\x02\
	\x02\u{106}\u{104}\x03\x02\x02\x02\u{106}\u{107}\x03\x02\x02\x02\u{107}\
	\x25\x03\x02\x02\x02\u{108}\u{106}\x03\x02\x02\x02\u{109}\u{10e}\x05\x28\
	\x15\x02\u{10a}\u{10b}\x07\x1f\x02\x02\u{10b}\u{10d}\x05\x28\x15\x02\u{10c}\
	\u{10a}\x03\x02\x02\x02\u{10d}\u{110}\x03\x02\x02\x02\u{10e}\u{10c}\x03\
	\x02\x02\x02\u{10e}\u{10f}\x03\x02\x02\x02\u{10f}\x27\x03\x02\x02\x02\u{110}\
	\u{10e}\x03\x02\x02\x02\u{111}\u{116}\x05\x2a\x16\x02\u{112}\u{113}\x07\
	\x20\x02\x02\u{113}\u{115}\x05\x2a\x16\x02\u{114}\u{112}\x03\x02\x02\x02\
	\u{115}\u{118}\x03\x02\x02\x02\u{116}\u{114}\x03\x02\x02\x02\u{116}\u{117}\
	\x03\x02\x02\x02\u{117}\x29\x03\x02\x02\x02\u{118}\u{116}\x03\x02\x02\x02\
	\u{119}\u{11c}\x05\x2c\x17\x02\u{11a}\u{11b}\x09\x04\x02\x02\u{11b}\u{11d}\
	\x05\x2c\x17\x02\u{11c}\u{11a}\x03\x02\x02\x02\u{11c}\u{11d}\x03\x02\x02\
	\x02\u{11d}\x2b\x03\x02\x02\x02\u{11e}\u{121}\x05\x2e\x18\x02\u{11f}\u{120}\
	\x09\x05\x02\x02\u{120}\u{122}\x05\x2e\x18\x02\u{121}\u{11f}\x03\x02\x02\
	\x02\u{121}\u{122}\x03\x02\x02\x02\u{122}\x2d\x03\x02\x02\x02\u{123}\u{128}\
	\x05\x30\x19\x02\u{124}\u{125}\x09\x06\x02\x02\u{125}\u{127}\x05\x30\x19\
	\x02\u{126}\u{124}\x03\x02\x02\x02\u{127}\u{12a}\x03\x02\x02\x02\u{128}\
	\u{126}\x03\x02\x02\x02\u{128}\u{129}\x03\x02\x02\x02\u{129}\x2f\x03\x02\
	\x02\x02\u{12a}\u{128}\x03\x02\x02\x02\u{12b}\u{130}\x05\x32\x1a\x02\u{12c}\
	\u{12d}\x09\x07\x02\x02\u{12d}\u{12f}\x05\x32\x1a\x02\u{12e}\u{12c}\x03\
	\x02\x02\x02\u{12f}\u{132}\x03\x02\x02\x02\u{130}\u{12e}\x03\x02\x02\x02\
	\u{130}\u{131}\x03\x02\x02\x02\u{131}\x31\x03\x02\x02\x02\u{132}\u{130}\
	\x03\x02\x02\x02\u{133}\u{134}\x09\x08\x02\x02\u{134}\u{137}\x05\x32\x1a\
	\x02\u{135}\u{137}\x05\x34\x1b\x02\u{136}\u{133}\x03\x02\x02\x02\u{136}\
	\u{135}\x03\x02\x02\x02\u{137}\x33\x03\x02\x02\x02\u{138}\u{13d}\x05\x36\
	\x1c\x02\u{139}\u{13a}\x07\x43\x02\x02\u{13a}\u{13e}\x05\x50\x29\x02\u{13b}\
	\u{13c}\x07\x44\x02\x02\u{13c}\u{13e}\x05\x34\x1b\x02\u{13d}\u{139}\x03\
	\x02\x02\x02\u{13d}\u{13b}\x03\x02\x02\x02\u{13d}\u{13e}\x03\x02\x02\x02\
	\u{13e}\x35\x03\x02\x02\x02\u{13f}\u{140}\x07\x2e\x02\x02\u{140}\u{141}\
	\x05\x1e\x10\x02\u{141}\u{145}\x07\x2f\x02\x02\u{142}\u{144}\x05\x3a\x1e\
	\x02\u{143}\u{142}\x03\x02\x02\x02\u{144}\u{147}\x03\x02\x02\x02\u{145}\
	\u{143}\x03\x02\x02\x02\u{145}\u{146}\x03\x02\x02\x02\u{146}\u{193}\x03\
	\x02\x02\x02\u{147}\u{145}\x03\x02\x02\x02\u{148}\u{14c}\x05\x3e\x20\x02\
	\u{149}\u{14b}\x05\x3a\x1e\x02\u{14a}\u{149}\x03\x02\x02\x02\u{14b}\u{14e}\
	\x03\x02\x02\x02\u{14c}\u{14a}\x03\x02\x02\x02\u{14c}\u{14d}\x03\x02\x02\
	\x02\u{14d}\u{193}\x03\x02\x02\x02\u{14e}\u{14c}\x03\x02\x02\x02\u{14f}\
	\u{154}\x05\x56\x2c\x02\u{150}\u{151}\x07\x04\x02\x02\u{151}\u{153}\x05\
	\x56\x2c\x02\u{152}\u{150}\x03\x02\x02\x02\u{153}\u{156}\x03\x02\x02\x02\
	\u{154}\u{152}\x03\x02\x02\x02\u{154}\u{155}\x03\x02\x02\x02\u{155}\u{157}\
	\x03\x02\x02\x02\u{156}\u{154}\x03\x02\x02\x02\u{157}\u{15b}\x05\x38\x1d\
	\x02\u{158}\u{15a}\x05\x3a\x1e\x02\u{159}\u{158}\x03\x02\x02\x02\u{15a}\
	\u{15d}\x03\x02\x02\x02\u{15b}\u{159}\x03\x02\x02\x02\u{15b}\u{15c}\x03\
	\x02\x02\x02\u{15c}\u{193}\x03\x02\x02\x02\u{15d}\u{15b}\x03\x02\x02\x02\
	\u{15e}\u{15f}\x07\x24\x02\x02\u{15f}\u{164}\x05\x56\x2c\x02\u{160}\u{161}\
	\x07\x04\x02\x02\u{161}\u{163}\x05\x56\x2c\x02\u{162}\u{160}\x03\x02\x02\
	\x02\u{163}\u{166}\x03\x02\x02\x02\u{164}\u{162}\x03\x02\x02\x02\u{164}\
	\u{165}\x03\x02\x02\x02\u{165}\u{168}\x03\x02\x02\x02\u{166}\u{164}\x03\
	\x02\x02\x02\u{167}\u{169}\x05\x38\x1d\x02\u{168}\u{167}\x03\x02\x02\x02\
	\u{168}\u{169}\x03\x02\x02\x02\u{169}\u{16a}\x03\x02\x02\x02\u{16a}\u{16e}\
	\x07\x26\x02\x02\u{16b}\u{16d}\x05\x3a\x1e\x02\u{16c}\u{16b}\x03\x02\x02\
	\x02\u{16d}\u{170}\x03\x02\x02\x02\u{16e}\u{16c}\x03\x02\x02\x02\u{16e}\
	\u{16f}\x03\x02\x02\x02\u{16f}\u{193}\x03\x02\x02\x02\u{170}\u{16e}\x03\
	\x02\x02\x02\u{171}\u{172}\x07\x30\x02\x02\u{172}\u{177}\x05\x56\x2c\x02\
	\u{173}\u{174}\x07\x04\x02\x02\u{174}\u{176}\x05\x56\x2c\x02\u{175}\u{173}\
	\x03\x02\x02\x02\u{176}\u{179}\x03\x02\x02\x02\u{177}\u{175}\x03\x02\x02\
	\x02\u{177}\u{178}\x03\x02\x02\x02\u{178}\u{17b}\x03\x02\x02\x02\u{179}\
	\u{177}\x03\x02\x02\x02\u{17a}\u{17c}\x05\x38\x1d\x02\u{17b}\u{17a}\x03\
	\x02\x02\x02\u{17b}\u{17c}\x03\x02\x02\x02\u{17c}\u{17d}\x03\x02\x02\x02\
	\u{17d}\u{181}\x07\x26\x02\x02\u{17e}\u{180}\x05\x3a\x1e\x02\u{17f}\u{17e}\
	\x03\x02\x02\x02\u{180}\u{183}\x03\x02\x02\x02\u{181}\u{17f}\x03\x02\x02\
	\x02\u{181}\u{182}\x03\x02\x02\x02\u{182}\u{193}\x03\x02\x02\x02\u{183}\
	\u{181}\x03\x02\x02\x02\u{184}\u{188}\x05\x56\x2c\x02\u{185}\u{187}\x05\
	\x3a\x1e\x02\u{186}\u{185}\x03\x02\x02\x02\u{187}\u{18a}\x03\x02\x02\x02\
	\u{188}\u{186}\x03\x02\x02\x02\u{188}\u{189}\x03\x02\x02\x02\u{189}\u{193}\
	\x03\x02\x02\x02\u{18a}\u{188}\x03\x02\x02\x02\u{18b}\u{18f}\x07\x31\x02\
	\x02\u{18c}\u{18e}\x05\x3a\x1e\x02\u{18d}\u{18c}\x03\x02\x02\x02\u{18e}\
	\u{191}\x03\x02\x02\x02\u{18f}\u{18d}\x03\x02\x02\x02\u{18f}\u{190}\x03\
	\x02\x02\x02\u{190}\u{193}\x03\x02\x02\x02\u{191}\u{18f}\x03\x02\x02\x02\
	\u{192}\u{13f}\x03\x02\x02\x02\u{192}\u{148}\x03\x02\x02\x02\u{192}\u{14f}\
	\x03\x02\x02\x02\u{192}\u{15e}\x03\x02\x02\x02\u{192}\u{171}\x03\x02\x02\
	\x02\u{192}\u{184}\x03\x02\x02\x02\u{192}\u{18b}\x03\x02\x02\x02\u{193}\
	\x37\x03\x02\x02\x02\u{194}\u{19d}\x07\x2e\x02\x02\u{195}\u{19a}\x05\x1e\
	\x10\x02\u{196}\u{197}\x07\x1b\x02\x02\u{197}\u{199}\x05\x1e\x10\x02\u{198}\
	\u{196}\x03\x02\x02\x02\u{199}\u{19c}\x03\x02\x02\x02\u{19a}\u{198}\x03\
	\x02\x02\x02\u{19a}\u{19b}\x03\x02\x02\x02\u{19b}\u{19e}\x03\x02\x02\x02\
	\u{19c}\u{19a}\x03\x02\x02\x02\u{19d}\u{195}\x03\x02\x02\x02\u{19d}\u{19e}\
	\x03\x02\x02\x02\u{19e}\u{19f}\x03\x02\x02\x02\u{19f}\u{1a0}\x07\x2f\x02\
	\x02\u{1a0}\x39\x03\x02\x02\x02\u{1a1}\u{1c1}\x07\x04\x02\x02\u{1a2}\u{1c2}\
	\x05\x56\x2c\x02\u{1a3}\u{1c2}\x07\x49\x02\x02\u{1a4}\u{1c2}\x07\x05\x02\
	\x02\u{1a5}\u{1a6}\x07\x24\x02\x02\u{1a6}\u{1ab}\x05\x56\x2c\x02\u{1a7}\
	\u{1a8}\x07\x04\x02\x02\u{1a8}\u{1aa}\x05\x56\x2c\x02\u{1a9}\u{1a7}\x03\
	\x02\x02\x02\u{1aa}\u{1ad}\x03\x02\x02\x02\u{1ab}\u{1a9}\x03\x02\x02\x02\
	\u{1ab}\u{1ac}\x03\x02\x02\x02\u{1ac}\u{1af}\x03\x02\x02\x02\u{1ad}\u{1ab}\
	\x03\x02\x02\x02\u{1ae}\u{1b0}\x05\x38\x1d\x02\u{1af}\u{1ae}\x03\x02\x02\
	\x02\u{1af}\u{1b0}\x03\x02\x02\x02\u{1b0}\u{1b1}\x03\x02\x02\x02\u{1b1}\
	\u{1b2}\x07\x26\x02\x02\u{1b2}\u{1c2}\x03\x02\x02\x02\u{1b3}\u{1b4}\x07\
	\x30\x02\x02\u{1b4}\u{1b9}\x05\x56\x2c\x02\u{1b5}\u{1b6}\x07\x04\x02\x02\
	\u{1b6}\u{1b8}\x05\x56\x2c\x02\u{1b7}\u{1b5}\x03\x02\x02\x02\u{1b8}\u{1bb}\
	\x03\x02\x02\x02\u{1b9}\u{1b7}\x03\x02\x02\x02\u{1b9}\u{1ba}\x03\x02\x02\
	\x02\u{1ba}\u{1bd}\x03\x02\x02\x02\u{1bb}\u{1b9}\x03\x02\x02\x02\u{1bc}\
	\u{1be}\x05\x38\x1d\x02\u{1bd}\u{1bc}\x03\x02\x02\x02\u{1bd}\u{1be}\x03\
	\x02\x02\x02\u{1be}\u{1bf}\x03\x02\x02\x02\u{1bf}\u{1c0}\x07\x26\x02\x02\
	\u{1c0}\u{1c2}\x03\x02\x02\x02\u{1c1}\u{1a2}\x03\x02\x02\x02\u{1c1}\u{1a3}\
	\x03\x02\x02\x02\u{1c1}\u{1a4}\x03\x02\x02\x02\u{1c1}\u{1a5}\x03\x02\x02\
	\x02\u{1c1}\u{1b3}\x03\x02\x02\x02\u{1c2}\u{1db}\x03\x02\x02\x02\u{1c3}\
	\u{1c4}\x07\x32\x02\x02\u{1c4}\u{1c5}\x05\x3c\x1f\x02\u{1c5}\u{1c6}\x07\
	\x33\x02\x02\u{1c6}\u{1db}\x03\x02\x02\x02\u{1c7}\u{1d8}\x07\x34\x02\x02\
	\u{1c8}\u{1cd}\x05\x56\x2c\x02\u{1c9}\u{1ca}\x07\x32\x02\x02\u{1ca}\u{1cb}\
	\x07\x49\x02\x02\u{1cb}\u{1cd}\x07\x33\x02\x02\u{1cc}\u{1c8}\x03\x02\x02\
	\x02\u{1cc}\u{1c9}\x03\x02\x02\x02\u{1cd}\u{1d9}\x03\x02\x02\x02\u{1ce}\
	\u{1d3}\x07\x05\x02\x02\u{1cf}\u{1d0}\x07\x32\x02\x02\u{1d0}\u{1d1}\x07\
	\x05\x02\x02\u{1d1}\u{1d3}\x07\x33\x02\x02\u{1d2}\u{1ce}\x03\x02\x02\x02\
	\u{1d2}\u{1cf}\x03\x02\x02\x02\u{1d3}\u{1d9}\x03\x02\x02\x02\u{1d4}\u{1d5}\
	\x07\x32\x02\x02\u{1d5}\u{1d6}\x05\x54\x2b\x02\u{1d6}\u{1d7}\x07\x33\x02\
	\x02\u{1d7}\u{1d9}\x03\x02\x02\x02\u{1d8}\u{1cc}\x03\x02\x02\x02\u{1d8}\
	\u{1d2}\x03\x02\x02\x02\u{1d8}\u{1d4}\x03\x02\x02\x02\u{1d9}\u{1db}\x03\
	\x02\x02\x02\u{1da}\u{1a1}\x03\x02\x02\x02\u{1da}\u{1c3}\x03\x02\x02\x02\
	\u{1da}\u{1c7}\x03\x02\x02\x02\u{1db}\x3b\x03\x02\x02\x02\u{1dc}\u{204}\
	\x07\x49\x02\x02\u{1dd}\u{204}\x07\x05\x02\x02\u{1de}\u{204}\x05\x54\x2b\
	\x02\u{1df}\u{1e1}\x05\x54\x2b\x02\u{1e0}\u{1df}\x03\x02\x02\x02\u{1e0}\
	\u{1e1}\x03\x02\x02\x02\u{1e1}\u{1e2}\x03\x02\x02\x02\u{1e2}\u{1e4}\x07\
	\x35\x02\x02\u{1e3}\u{1e5}\x05\x54\x2b\x02\u{1e4}\u{1e3}\x03\x02\x02\x02\
	\u{1e4}\u{1e5}\x03\x02\x02\x02\u{1e5}\u{1e8}\x03\x02\x02\x02\u{1e6}\u{1e7}\
	\x07\x35\x02\x02\u{1e7}\u{1e9}\x05\x54\x2b\x02\u{1e8}\u{1e6}\x03\x02\x02\
	\x02\u{1e8}\u{1e9}\x03\x02\x02\x02\u{1e9}\u{204}\x03\x02\x02\x02\u{1ea}\
	\u{1eb}\x07\x2e\x02\x02\u{1eb}\u{1ec}\x05\x1e\x10\x02\u{1ec}\u{1ed}\x07\
	\x2f\x02\x02\u{1ed}\u{204}\x03\x02\x02\x02\u{1ee}\u{1ef}\x07\x36\x02\x02\
	\u{1ef}\u{1f0}\x07\x2e\x02\x02\u{1f0}\u{1f1}\x05\x1e\x10\x02\u{1f1}\u{1f2}\
	\x07\x2f\x02\x02\u{1f2}\u{204}\x03\x02\x02\x02\u{1f3}\u{1f8}\x05\x54\x2b\
	\x02\u{1f4}\u{1f5}\x07\x1b\x02\x02\u{1f5}\u{1f7}\x05\x54\x2b\x02\u{1f6}\
	\u{1f4}\x03\x02\x02\x02\u{1f7}\u{1fa}\x03\x02\x02\x02\u{1f8}\u{1f6}\x03\
	\x02\x02\x02\u{1f8}\u{1f9}\x03\x02\x02\x02\u{1f9}\u{204}\x03\x02\x02\x02\
	\u{1fa}\u{1f8}\x03\x02\x02\x02\u{1fb}\u{200}\x07\x49\x02\x02\u{1fc}\u{1fd}\
	\x07\x1b\x02\x02\u{1fd}\u{1ff}\x07\x49\x02\x02\u{1fe}\u{1fc}\x03\x02\x02\
	\x02\u{1ff}\u{202}\x03\x02\x02\x02\u{200}\u{1fe}\x03\x02\x02\x02\u{200}\
	\u{201}\x03\x02\x02\x02\u{201}\u{204}\x03\x02\x02\x02\u{202}\u{200}\x03\
	\x02\x02\x02\u{203}\u{1dc}\x03\x02\x02\x02\u{203}\u{1dd}\x03\x02\x02\x02\
	\u{203}\u{1de}\x03\x02\x02\x02\u{203}\u{1e0}\x03\x02\x02\x02\u{203}\u{1ea}\
	\x03\x02\x02\x02\u{203}\u{1ee}\x03\x02\x02\x02\u{203}\u{1f3}\x03\x02\x02\
	\x02\u{203}\u{1fb}\x03\x02\x02\x02\u{204}\x3d\x03\x02\x02\x02\u{205}\u{20d}\
	\x05\x40\x21\x02\u{206}\u{20d}\x05\x44\x23\x02\u{207}\u{20d}\x05\x4e\x28\
	\x02\u{208}\u{20d}\x05\x4c\x27\x02\u{209}\u{20d}\x05\x46\x24\x02\u{20a}\
	\u{20d}\x05\x48\x25\x02\u{20b}\u{20d}\x05\x4a\x26\x02\u{20c}\u{205}\x03\
	\x02\x02\x02\u{20c}\u{206}\x03\x02\x02\x02\u{20c}\u{207}\x03\x02\x02\x02\
	\u{20c}\u{208}\x03\x02\x02\x02\u{20c}\u{209}\x03\x02\x02\x02\u{20c}\u{20a}\
	\x03\x02\x02\x02\u{20c}\u{20b}\x03\x02\x02\x02\u{20d}\x3f\x03\x02\x02\x02\
	\u{20e}\u{217}\x07\x37\x02\x02\u{20f}\u{214}\x05\x42\x22\x02\u{210}\u{211}\
	\x07\x1b\x02\x02\u{211}\u{213}\x05\x42\x22\x02\u{212}\u{210}\x03\x02\x02\
	\x02\u{213}\u{216}\x03\x02\x02\x02\u{214}\u{212}\x03\x02\x02\x02\u{214}\
	\u{215}\x03\x02\x02\x02\u{215}\u{218}\x03\x02\x02\x02\u{216}\u{214}\x03\
	\x02\x02\x02\u{217}\u{20f}\x03\x02\x02\x02\u{217}\u{218}\x03\x02\x02\x02\
	\u{218}\u{219}\x03\x02\x02\x02\u{219}\u{21a}\x07\x38\x02\x02\u{21a}\x41\
	\x03\x02\x02\x02\u{21b}\u{21c}\x07\x49\x02\x02\u{21c}\u{21d}\x07\x35\x02\
	\x02\u{21d}\u{21e}\x05\x1e\x10\x02\u{21e}\x43\x03\x02\x02\x02\u{21f}\u{228}\
	\x07\x32\x02\x02\u{220}\u{225}\x05\x1e\x10\x02\u{221}\u{222}\x07\x1b\x02\
	\x02\u{222}\u{224}\x05\x1e\x10\x02\u{223}\u{221}\x03\x02\x02\x02\u{224}\
	\u{227}\x03\x02\x02\x02\u{225}\u{223}\x03\x02\x02\x02\u{225}\u{226}\x03\
	\x02\x02\x02\u{226}\u{229}\x03\x02\x02\x02\u{227}\u{225}\x03\x02\x02\x02\
	\u{228}\u{220}\x03\x02\x02\x02\u{228}\u{229}\x03\x02\x02\x02\u{229}\u{22a}\
	\x03\x02\x02\x02\u{22a}\u{22b}\x07\x33\x02\x02\u{22b}\x45\x03\x02\x02\x02\
	\u{22c}\u{22d}\x09\x09\x02\x02\u{22d}\x47\x03\x02\x02\x02\u{22e}\u{22f}\
	\x07\x3b\x02\x02\u{22f}\x49\x03\x02\x02\x02\u{230}\u{231}\x07\x3c\x02\x02\
	\u{231}\x4b\x03\x02\x02\x02\u{232}\u{233}\x07\x49\x02\x02\u{233}\x4d\x03\
	\x02\x02\x02\u{234}\u{235}\x07\x45\x02\x02\u{235}\x4f\x03\x02\x02\x02\u{236}\
	\u{238}\x07\x3d\x02\x02\u{237}\u{236}\x03\x02\x02\x02\u{237}\u{238}\x03\
	\x02\x02\x02\u{238}\u{239}\x03\x02\x02\x02\u{239}\u{23e}\x05\x56\x2c\x02\
	\u{23a}\u{23b}\x07\x04\x02\x02\u{23b}\u{23d}\x05\x56\x2c\x02\u{23c}\u{23a}\
	\x03\x02\x02\x02\u{23d}\u{240}\x03\x02\x02\x02\u{23e}\u{23c}\x03\x02\x02\
	\x02\u{23e}\u{23f}\x03\x02\x02\x02\u{23f}\u{242}\x03\x02\x02\x02\u{240}\
	\u{23e}\x03\x02\x02\x02\u{241}\u{243}\x05\x38\x1d\x02\u{242}\u{241}\x03\
	\x02\x02\x02\u{242}\u{243}\x03\x02\x02\x02\u{243}\u{250}\x03\x02\x02\x02\
	\u{244}\u{245}\x07\x37\x02\x02\u{245}\u{24a}\x05\x52\x2a\x02\u{246}\u{247}\
	\x07\x1b\x02\x02\u{247}\u{249}\x05\x52\x2a\x02\u{248}\u{246}\x03\x02\x02\
	\x02\u{249}\u{24c}\x03\x02\x02\x02\u{24a}\u{248}\x03\x02\x02\x02\u{24a}\
	\u{24b}\x03\x02\x02\x02\u{24b}\u{24d}\x03\x02\x02\x02\u{24c}\u{24a}\x03\
	\x02\x02\x02\u{24d}\u{24e}\x07\x38\x02\x02\u{24e}\u{250}\x03\x02\x02\x02\
	\u{24f}\u{237}\x03\x02\x02\x02\u{24f}\u{244}\x03\x02\x02\x02\u{250}\x51\
	\x03\x02\x02\x02\u{251}\u{253}\x07\x3d\x02\x02\u{252}\u{251}\x03\x02\x02\
	\x02\u{252}\u{253}\x03\x02\x02\x02\u{253}\u{255}\x03\x02\x02\x02\u{254}\
	\u{256}\x07\x31\x02\x02\u{255}\u{254}\x03\x02\x02\x02\u{255}\u{256}\x03\
	\x02\x02\x02\u{256}\u{257}\x03\x02\x02\x02\u{257}\u{258}\x07\x35\x02\x02\
	\u{258}\u{25d}\x05\x56\x2c\x02\u{259}\u{25a}\x07\x04\x02\x02\u{25a}\u{25c}\
	\x05\x56\x2c\x02\u{25b}\u{259}\x03\x02\x02\x02\u{25c}\u{25f}\x03\x02\x02\
	\x02\u{25d}\u{25b}\x03\x02\x02\x02\u{25d}\u{25e}\x03\x02\x02\x02\u{25e}\
	\u{261}\x03\x02\x02\x02\u{25f}\u{25d}\x03\x02\x02\x02\u{260}\u{262}\x05\
	\x38\x1d\x02\u{261}\u{260}\x03\x02\x02\x02\u{261}\u{262}\x03\x02\x02\x02\
	\u{262}\x53\x03\x02\x02\x02\u{263}\u{265}\x07\x2a\x02\x02\u{264}\u{263}\
	\x03\x02\x02\x02\u{264}\u{265}\x03\x02\x02\x02\u{265}\u{266}\x03\x02\x02\
	\x02\u{266}\u{267}\x07\x45\x02\x02\u{267}\x55\x03\x02\x02\x02\u{268}\u{26b}\
	\x07\x42\x02\x02\u{269}\u{26b}\x05\x58\x2d\x02\u{26a}\u{268}\x03\x02\x02\
	\x02\u{26a}\u{269}\x03\x02\x02\x02\u{26b}\x57\x03\x02\x02\x02\u{26c}\u{26d}\
	\x09\x0a\x02\x02\u{26d}\x59\x03\x02\x02\x02\x4f\x5d\x63\x6d\x74\x7f\u{8a}\
	\u{92}\u{99}\u{a0}\u{a7}\u{ad}\u{b5}\u{bc}\u{bf}\u{c5}\u{cc}\u{d3}\u{dc}\
	\u{ea}\u{ed}\u{f6}\u{fe}\u{106}\u{10e}\u{116}\u{11c}\u{121}\u{128}\u{130}\
	\u{136}\u{13d}\u{145}\u{14c}\u{154}\u{15b}\u{164}\u{168}\u{16e}\u{177}\u{17b}\
	\u{181}\u{188}\u{18f}\u{192}\u{19a}\u{19d}\u{1ab}\u{1af}\u{1b9}\u{1bd}\u{1c1}\
	\u{1cc}\u{1d2}\u{1d8}\u{1da}\u{1e0}\u{1e4}\u{1e8}\u{1f8}\u{200}\u{203}\u{20c}\
	\u{214}\u{217}\u{225}\u{228}\u{237}\u{23e}\u{242}\u{24a}\u{24f}\u{252}\u{255}\
	\u{25d}\u{261}\u{264}\u{26a}";

