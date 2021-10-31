mod custom;
pub mod margin_padding;
pub mod background;
pub mod outline;
pub mod flex;
pub mod align;
pub mod font;
pub mod box_shadow;
pub mod border;
pub mod border_image;
pub mod border_radius;
pub mod transition;
pub mod animation;
pub mod transform;

use cssparser::*;
use custom::*;
use background::*;
use outline::*;
use flex::*;
use align::*;
use font::*;
use box_shadow::*;
use border::*;
use border_image::*;
use border_radius::*;
use transition::*;
use animation::*;
use transform::*;
use crate::values::{image::*, length::*, rect::*, color::*, time::Time, ident::CustomIdent, easing::EasingFunction};
use crate::traits::{Parse, ToCss};
use crate::printer::Printer;
use std::fmt::Write;
use smallvec::{SmallVec, smallvec};

#[derive(Debug, Clone, PartialEq)]
pub enum Property {
  BackgroundColor(CssColor),
  BackgroundImage(SmallVec<[Image; 1]>),
  BackgroundPositionX(SmallVec<[HorizontalPosition; 1]>),
  BackgroundPositionY(SmallVec<[VerticalPosition; 1]>),
  BackgroundPosition(SmallVec<[BackgroundPosition; 1]>),
  BackgroundSize(SmallVec<[BackgroundSize; 1]>),
  BackgroundRepeat(SmallVec<[BackgroundRepeat; 1]>),
  BackgroundAttachment(SmallVec<[BackgroundAttachment; 1]>),
  BackgroundClip(SmallVec<[BackgroundBox; 1]>),
  BackgroundOrigin(SmallVec<[BackgroundBox; 1]>),
  // BackgroundBlendMode
  Background(SmallVec<[Background; 1]>),

  BoxShadow(SmallVec<[BoxShadow; 1]>),
  Opacity(AlphaValue),

  Color(CssColor),
  Custom(CustomProperty),

  Width(Size),
  Height(Size),
  MinWidth(MinMaxSize),
  MinHeight(MinMaxSize),
  MaxWidth(MinMaxSize),
  MaxHeight(MinMaxSize),
  BlockSize(Size),
  InlineSize(Size),
  MinBlockSize(MinMaxSize),
  MinInlineSize(MinMaxSize),
  MaxBlockSize(MinMaxSize),
  MaxInlineSize(MinMaxSize),

  Top(LengthPercentageOrAuto),
  Bottom(LengthPercentageOrAuto),
  Left(LengthPercentageOrAuto),
  Right(LengthPercentageOrAuto),
  InsetBlockStart(LengthPercentageOrAuto),
  InsetBlockEnd(LengthPercentageOrAuto),
  InsetInlineStart(LengthPercentageOrAuto),
  InsetInlineEnd(LengthPercentageOrAuto),

  BorderTopColor(CssColor),
  BorderBottomColor(CssColor),
  BorderLeftColor(CssColor),
  BorderRightColor(CssColor),
  BorderBlockStartColor(CssColor),
  BorderBlockEndColor(CssColor),
  BorderInlineStartColor(CssColor),
  BorderInlineEndColor(CssColor),

  BorderTopStyle(BorderStyle),
  BorderBottomStyle(BorderStyle),
  BorderLeftStyle(BorderStyle),
  BorderRightStyle(BorderStyle),
  BorderBlockStartStyle(BorderStyle),
  BorderBlockEndStyle(BorderStyle),
  BorderInlineStartStyle(BorderStyle),
  BorderInlineEndStyle(BorderStyle),

  BorderTopWidth(BorderSideWidth),
  BorderBottomWidth(BorderSideWidth),
  BorderLeftWidth(BorderSideWidth),
  BorderRightWidth(BorderSideWidth),
  BorderBlockStartWidth(BorderSideWidth),
  BorderBlockEndWidth(BorderSideWidth),
  BorderInlineStartWidth(BorderSideWidth),
  BorderInlineEndWidth(BorderSideWidth),

  BorderTopLeftRadius(Size2D<LengthPercentage>),
  BorderTopRightRadius(Size2D<LengthPercentage>),
  BorderBottomLeftRadius(Size2D<LengthPercentage>),
  BorderBottomRightRadius(Size2D<LengthPercentage>),
  BorderStartStartRadius(Size2D<LengthPercentage>),
  BorderStartEndRadius(Size2D<LengthPercentage>),
  BorderEndStartRadius(Size2D<LengthPercentage>),
  BorderEndEndRadius(Size2D<LengthPercentage>),
  BorderRadius(BorderRadius),

  /// https://www.w3.org/TR/css-backgrounds-3/#border-image-source
  BorderImageSource(Image),
  /// https://www.w3.org/TR/css-backgrounds-3/#border-image-outset
  BorderImageOutset(Rect<LengthOrNumber>),
  /// https://www.w3.org/TR/css-backgrounds-3/#border-image-repeat
  BorderImageRepeat(BorderImageRepeat),
  /// https://www.w3.org/TR/css-backgrounds-3/#border-image-width
  BorderImageWidth(Rect<BorderImageSideWidth>),
  /// https://www.w3.org/TR/css-backgrounds-3/#border-image-slice
  BorderImageSlice(BorderImageSlice),
  /// https://www.w3.org/TR/css-backgrounds-3/#border-image
  BorderImage(BorderImage),

  BorderColor(Rect<CssColor>),
  BorderStyle(Rect<BorderStyle>),
  BorderWidth(Rect<BorderSideWidth>),

  BorderBlockColor(CssColor),
  BorderBlockStyle(BorderStyle),
  BorderBlockWidth(BorderSideWidth),

  BorderInlineColor(CssColor),
  BorderInlineStyle(BorderStyle),
  BorderInlineWidth(BorderSideWidth),

  Border(Border),
  BorderTop(Border),
  BorderBottom(Border),
  BorderLeft(Border),
  BorderRight(Border),
  BorderBlock(Border),
  BorderBlockStart(Border),
  BorderBlockEnd(Border),
  BorderInline(Border),
  BorderInlineStart(Border),
  BorderInlineEnd(Border),

  Outline(Outline),
  OutlineColor(CssColor),
  OutlineStyle(OutlineStyle),
  OutlineWidth(BorderSideWidth),

  // Flex properties: https://www.w3.org/TR/2018/CR-css-flexbox-1-20181119
  FlexDirection(FlexDirection),
  FlexWrap(FlexWrap),
  FlexFlow(FlexFlow),
  FlexGrow(f32),
  FlexShrink(f32),
  FlexBasis(LengthPercentageOrAuto),
  Flex(Flex),

  // Align properties: https://www.w3.org/TR/2020/WD-css-align-3-20200421
  AlignContent(AlignContent),
  JustifyContent(JustifyContent),
  PlaceContent(PlaceContent),
  AlignSelf(AlignSelf),
  JustifySelf(JustifySelf),
  PlaceSelf(PlaceSelf),
  AlignItems(AlignItems),
  JustifyItems(JustifyItems),
  PlaceItems(PlaceItems),
  RowGap(GapValue),
  ColumnGap(GapValue),
  Gap(Gap),

  MarginTop(LengthPercentageOrAuto),
  MarginBottom(LengthPercentageOrAuto),
  MarginLeft(LengthPercentageOrAuto),
  MarginRight(LengthPercentageOrAuto),
  MarginBlockStart(LengthPercentageOrAuto),
  MarginBlockEnd(LengthPercentageOrAuto),
  MarginInlineStart(LengthPercentageOrAuto),
  MarginInlineEnd(LengthPercentageOrAuto),
  MarginBlock(Size2D<LengthPercentageOrAuto>),
  MarginInline(Size2D<LengthPercentageOrAuto>),
  Margin(Rect<LengthPercentageOrAuto>),

  PaddingTop(LengthPercentageOrAuto),
  PaddingBottom(LengthPercentageOrAuto),
  PaddingLeft(LengthPercentageOrAuto),
  PaddingRight(LengthPercentageOrAuto),
  PaddingBlockStart(LengthPercentageOrAuto),
  PaddingBlockEnd(LengthPercentageOrAuto),
  PaddingInlineStart(LengthPercentageOrAuto),
  PaddingInlineEnd(LengthPercentageOrAuto),
  PaddingBlock(Size2D<LengthPercentageOrAuto>),
  PaddingInline(Size2D<LengthPercentageOrAuto>),
  Padding(Rect<LengthPercentageOrAuto>),

  ScrollMarginTop(LengthPercentageOrAuto),
  ScrollMarginBottom(LengthPercentageOrAuto),
  ScrollMarginLeft(LengthPercentageOrAuto),
  ScrollMarginRight(LengthPercentageOrAuto),
  ScrollMarginBlockStart(LengthPercentageOrAuto),
  ScrollMarginBlockEnd(LengthPercentageOrAuto),
  ScrollMarginInlineStart(LengthPercentageOrAuto),
  ScrollMarginInlineEnd(LengthPercentageOrAuto),
  ScrollMarginBlock(Size2D<LengthPercentageOrAuto>),
  ScrollMarginInline(Size2D<LengthPercentageOrAuto>),
  ScrollMargin(Rect<LengthPercentageOrAuto>),

  ScrollPaddingTop(LengthPercentageOrAuto),
  ScrollPaddingBottom(LengthPercentageOrAuto),
  ScrollPaddingLeft(LengthPercentageOrAuto),
  ScrollPaddingRight(LengthPercentageOrAuto),
  ScrollPaddingBlockStart(LengthPercentageOrAuto),
  ScrollPaddingBlockEnd(LengthPercentageOrAuto),
  ScrollPaddingInlineStart(LengthPercentageOrAuto),
  ScrollPaddingInlineEnd(LengthPercentageOrAuto),
  ScrollPaddingBlock(Size2D<LengthPercentageOrAuto>),
  ScrollPaddingInline(Size2D<LengthPercentageOrAuto>),
  ScrollPadding(Rect<LengthPercentageOrAuto>),

  // shorthands: transitions, animations, columns, font, font-variant, list-style
  // grid, inset

  FontWeight(FontWeight),
  FontSize(FontSize),
  FontStretch(FontStretch),
  FontFamily(Vec<FontFamily>),
  FontStyle(FontStyle),
  FontVariantCaps(FontVariantCaps),
  LineHeight(LineHeight),
  Font(Font),

  TransitionProperty(SmallVec<[CustomIdent; 1]>),
  TransitionDuration(SmallVec<[Time; 1]>),
  TransitionDelay(SmallVec<[Time; 1]>),
  TransitionTimingFunction(SmallVec<[EasingFunction; 1]>),
  Transition(SmallVec<[Transition; 1]>),

  AnimationName(SmallVec<[AnimationName; 1]>),
  AnimationDuration(SmallVec<[Time; 1]>),
  AnimationTimingFunction(SmallVec<[EasingFunction; 1]>),
  AnimationIterationCount(SmallVec<[AnimationIterationCount; 1]>),
  AnimationDirection(SmallVec<[AnimationDirection; 1]>),
  AnimationPlayState(SmallVec<[AnimationPlayState; 1]>),
  AnimationDelay(SmallVec<[Time; 1]>),
  AnimationFillMode(SmallVec<[AnimationFillMode; 1]>),
  Animation(SmallVec<[Animation; 1]>),

  Transform(TransformList)
}

impl Property {
  pub fn parse<'i, 't>(name: CowRcStr<'i>, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ()>> {
    macro_rules! property {
      // font-family is special because it uses Vec rather than SmallVec
      // It's likely that there will be more than one font family vs other
      // properties where only one is the common case.
      (FontFamily, $type: ident) => {
        if let Ok(c) = input.parse_comma_separated(|input| $type::parse(input)) {
          return Ok(Property::FontFamily(c))
        }
      };
      ($property: ident, $type: ident) => {
        if let Ok(c) = $type::parse(input) {
          return Ok(Property::$property(c))
        }
      };
      ($property: ident, $type: ident, $multi: expr) => {{
        // Copied from cssparser `parse_comma_separated` but using SmallVec instead of Vec.
        let mut values = smallvec![];
        loop {
          input.skip_whitespace(); // Unnecessary for correctness, but may help try() in parse_one rewind less.
          match input.parse_until_before(Delimiter::Comma, &mut $type::parse) {
            Ok(v) => values.push(v),
            Err(_) => break
          }
          match input.next() {
            Err(_) => return Ok(Property::$property(values)),
            Ok(&Token::Comma) => continue,
            Ok(_) => unreachable!(),
          }
        }
      }};
    }
    
    let state = input.state();
    match name.as_ref() {
      "background-color" => property!(BackgroundColor, CssColor),
      "background-image" => property!(BackgroundImage, Image, true),
      "background-position-x" => property!(BackgroundPositionX, HorizontalPosition, true),
      "background-position-y" => property!(BackgroundPositionY, VerticalPosition, true),
      "background-position" => property!(BackgroundPosition, BackgroundPosition, true),
      "background-size" => property!(BackgroundSize, BackgroundSize, true),
      "background-repeat" => property!(BackgroundRepeat, BackgroundRepeat, true),
      "background-attachment" => property!(BackgroundAttachment, BackgroundAttachment, true),
      "background-origin" => property!(BackgroundOrigin, BackgroundBox, true),
      "background-clip" => property!(BackgroundClip, BackgroundBox, true),
      "background" => property!(Background, Background, true),
      "box-shadow" => property!(BoxShadow, BoxShadow, true),
      "opacity" => property!(Opacity, AlphaValue),
      "color" => property!(Color, CssColor),
      "width" => property!(Width, Size),
      "height" => property!(Height, Size),
      "min-width" => property!(MinWidth, MinMaxSize),
      "min-height" => property!(MinHeight, MinMaxSize),
      "max-width" => property!(MaxWidth, MinMaxSize),
      "max-height" => property!(MaxHeight, MinMaxSize),
      "block-size" => property!(BlockSize, Size),
      "inline-size" => property!(InlineSize, Size),
      "min-block-size" => property!(MinBlockSize, MinMaxSize),
      "min-inline-size" => property!(MinInlineSize, MinMaxSize),
      "max-block-size" => property!(MaxBlockSize, MinMaxSize),
      "max-inline-size" => property!(MaxInlineSize, MinMaxSize),
      "top" => property!(Top, LengthPercentageOrAuto),
      "bottom" => property!(Bottom, LengthPercentageOrAuto),
      "left" => property!(Left, LengthPercentageOrAuto),
      "right" => property!(Right, LengthPercentageOrAuto),
      "inset-block-start" => property!(InsetBlockStart, LengthPercentageOrAuto),
      "inset-block-end" => property!(InsetBlockEnd, LengthPercentageOrAuto),
      "inset-inline-start" => property!(InsetInlineStart, LengthPercentageOrAuto),
      "inset-inline-end" => property!(InsetInlineEnd, LengthPercentageOrAuto),
      "border-top-color" => property!(BorderTopColor, CssColor),
      "border-bottom-color" => property!(BorderBottomColor, CssColor),
      "border-left-color" => property!(BorderLeftColor, CssColor),
      "border-right-color" => property!(BorderRightColor, CssColor),
      "border-block-start-color" => property!(BorderBlockStartColor, CssColor),
      "border-block-end-color" => property!(BorderBlockEndColor, CssColor),
      "border-inline-start-color" => property!(BorderInlineStartColor, CssColor),
      "border-inline-end-color" => property!(BorderInlineEndColor, CssColor),
      "border-top-style" => property!(BorderTopStyle, BorderStyle),
      "border-bottom-style" => property!(BorderBottomStyle, BorderStyle),
      "border-left-style" => property!(BorderLeftStyle, BorderStyle),
      "border-right-style" => property!(BorderRightStyle, BorderStyle),
      "border-block-start-style" => property!(BorderBlockStartStyle, BorderStyle),
      "border-block-end-style" => property!(BorderBlockEndStyle, BorderStyle),
      "border-inline-start-style" => property!(BorderInlineStartStyle, BorderStyle),
      "border-inline-end-style" => property!(BorderInlineEndStyle, BorderStyle),
      "border-top-width" => property!(BorderTopWidth, BorderSideWidth),
      "border-bottom-width" => property!(BorderBottomWidth, BorderSideWidth),
      "border-left-width" => property!(BorderLeftWidth, BorderSideWidth),
      "border-right-width" => property!(BorderRightWidth, BorderSideWidth),
      "border-block-start-width" => property!(BorderBlockStartWidth, BorderSideWidth),
      "border-block-end-width" => property!(BorderBlockEndWidth, BorderSideWidth),
      "border-inline-start-width" => property!(BorderInlineStartWidth, BorderSideWidth),
      "border-inline-end-width" => property!(BorderInlineEndWidth, BorderSideWidth),
      "border-color" => property!(BorderColor, Rect),
      "border-style" => property!(BorderStyle, Rect),
      "border-width" => property!(BorderWidth, Rect),
      "border-block-color" => property!(BorderBlockColor, CssColor),
      "border-block-style" => property!(BorderBlockStyle, BorderStyle),
      "border-block-width" => property!(BorderBlockWidth, BorderSideWidth),
      "border-inline-color" => property!(BorderInlineColor, CssColor),
      "border-inline-style" => property!(BorderInlineStyle, BorderStyle),
      "border-inline-width" => property!(BorderInlineWidth, BorderSideWidth),
      "border" => property!(Border, Border),
      "border-top" => property!(BorderTop, Border),
      "border-bottom" => property!(BorderBottom, Border),
      "border-left" => property!(BorderLeft, Border),
      "border-right" => property!(BorderRight, Border),
      "border-block" => property!(BorderBlock, Border),
      "border-block-start" => property!(BorderBlockStart, Border),
      "border-block-end" => property!(BorderBlockEnd, Border),
      "border-inline" => property!(BorderInline, Border),
      "border-inline-start" => property!(BorderInlineStart, Border),
      "border-inline-end" => property!(BorderInlineEnd, Border),
      "border-image-source" => property!(BorderImageSource, Image),
      "border-image-outset" => property!(BorderImageOutset, Rect),
      "border-image-repeat" => property!(BorderImageRepeat, BorderImageRepeat),
      "border-image-width" => property!(BorderImageWidth, Rect),
      "border-image-slice" => property!(BorderImageSlice, BorderImageSlice),
      "border-image" => property!(BorderImage, BorderImage),
      "border-top-left-radius" => property!(BorderTopLeftRadius, Size2D),
      "border-top-right-radius" => property!(BorderTopRightRadius, Size2D),
      "border-bottom-left-radius" => property!(BorderBottomLeftRadius, Size2D),
      "border-bottom-right-radius" => property!(BorderBottomRightRadius, Size2D),
      "border-start-start-radius" => property!(BorderStartStartRadius, Size2D),
      "border-start-end-radius" => property!(BorderStartEndRadius, Size2D),
      "border-end-start-radius" => property!(BorderEndStartRadius, Size2D),
      "border-end-end-radius" => property!(BorderEndEndRadius, Size2D),
      "border-radius" => property!(BorderRadius, BorderRadius),
      "outline" => property!(Outline, Outline),
      "outline-color" => property!(OutlineColor, CssColor),
      "outline-style" => property!(OutlineStyle, OutlineStyle),
      "outline-width" => property!(OutlineWidth, BorderSideWidth),
      "flex-direction" => property!(FlexDirection, FlexDirection),
      "flex-wrap" => property!(FlexWrap, FlexWrap),
      "flex-flow" => property!(FlexFlow, FlexFlow),
      "flex-grow" => property!(FlexGrow, f32),
      "flex-shrink" => property!(FlexShrink, f32),
      "flex-basis" => property!(FlexBasis, LengthPercentageOrAuto),
      "flex" => property!(Flex, Flex),
      "align-content" => property!(AlignContent, AlignContent),
      "justify-content" => property!(JustifyContent, JustifyContent),
      "place-content" => property!(PlaceContent, PlaceContent),
      "align-self" => property!(AlignSelf, AlignSelf),
      "justify-self" => property!(JustifySelf, JustifySelf),
      "place-self" => property!(PlaceSelf, PlaceSelf),
      "align-items" => property!(AlignItems, AlignItems),
      "justify-items" => property!(JustifyItems, JustifyItems),
      "place-items" => property!(PlaceItems, PlaceItems),
      "row-gap" => property!(RowGap, GapValue),
      "column-gap" => property!(ColumnGap, GapValue),
      "gap" => property!(Gap, Gap),
      // https://www.w3.org/TR/2020/WD-css-align-3-20200421/#gap-legacy
      "grid-row-gap" => property!(RowGap, GapValue),
      "grid-column-gap" => property!(ColumnGap, GapValue),
      "grid-gap" => property!(Gap, Gap),
      "margin-left" => property!(MarginLeft, LengthPercentageOrAuto),
      "margin-right" => property!(MarginRight, LengthPercentageOrAuto),
      "margin-top" => property!(MarginTop, LengthPercentageOrAuto),
      "margin-bottom" => property!(MarginBottom, LengthPercentageOrAuto),
      "margin-block-start" => property!(MarginBlockStart, LengthPercentageOrAuto),
      "margin-block-end" => property!(MarginBlockEnd, LengthPercentageOrAuto),
      "margin-inline-start" => property!(MarginInlineStart, LengthPercentageOrAuto),
      "margin-inline-end" => property!(MarginInlineEnd, LengthPercentageOrAuto),
      "margin-block" => property!(MarginBlock, Size2D),
      "margin-inline" => property!(MarginInline, Size2D),
      "margin" => property!(Margin, Rect),
      "padding-left" => property!(PaddingLeft, LengthPercentageOrAuto),
      "padding-right" => property!(PaddingRight, LengthPercentageOrAuto),
      "padding-top" => property!(PaddingTop, LengthPercentageOrAuto),
      "padding-bottom" => property!(PaddingBottom, LengthPercentageOrAuto),
      "padding-block-start" => property!(PaddingBlockStart, LengthPercentageOrAuto),
      "padding-block-end" => property!(PaddingBlockEnd, LengthPercentageOrAuto),
      "padding-inline-start" => property!(PaddingInlineStart, LengthPercentageOrAuto),
      "padding-inline-end" => property!(PaddingInlineEnd, LengthPercentageOrAuto),
      "padding-block" => property!(PaddingBlock, Size2D),
      "padding-inline" => property!(PaddingInline, Size2D),
      "padding" => property!(Padding, Rect),
      "scroll-margin-left" => property!(ScrollMarginLeft, LengthPercentageOrAuto),
      "scroll-margin-right" => property!(ScrollMarginRight, LengthPercentageOrAuto),
      "scroll-margin-top" => property!(ScrollMarginTop, LengthPercentageOrAuto),
      "scroll-margin-bottom" => property!(ScrollMarginBottom, LengthPercentageOrAuto),
      "scroll-margin-block-start" => property!(ScrollMarginBlockStart, LengthPercentageOrAuto),
      "scroll-margin-block-end" => property!(ScrollMarginBlockEnd, LengthPercentageOrAuto),
      "scroll-margin-inline-start" => property!(ScrollMarginInlineStart, LengthPercentageOrAuto),
      "scroll-margin-inline-end" => property!(ScrollMarginInlineEnd, LengthPercentageOrAuto),
      "scroll-margin-block" => property!(ScrollMarginBlock, Size2D),
      "scroll-margin-inline" => property!(ScrollMarginInline, Size2D),
      "scroll-margin" => property!(ScrollMargin, Rect),
      "scroll-padding-left" => property!(ScrollPaddingLeft, LengthPercentageOrAuto),
      "scroll-padding-right" => property!(ScrollPaddingRight, LengthPercentageOrAuto),
      "scroll-padding-top" => property!(ScrollPaddingTop, LengthPercentageOrAuto),
      "scroll-padding-bottom" => property!(ScrollPaddingBottom, LengthPercentageOrAuto),
      "scroll-padding-block-start" => property!(ScrollPaddingBlockStart, LengthPercentageOrAuto),
      "scroll-padding-block-end" => property!(ScrollPaddingBlockEnd, LengthPercentageOrAuto),
      "scroll-padding-inline-start" => property!(ScrollPaddingInlineStart, LengthPercentageOrAuto),
      "scroll-padding-inline-end" => property!(ScrollPaddingInlineEnd, LengthPercentageOrAuto),
      "scroll-padding-block" => property!(ScrollPaddingBlock, Size2D),
      "scroll-padding-inline" => property!(ScrollPaddingInline, Size2D),
      "scroll-padding" => property!(ScrollPadding, Rect),
      "font-weight" => property!(FontWeight, FontWeight),
      "font-size" => property!(FontSize, FontSize),
      "font-stretch" => property!(FontStretch, FontStretch),
      "font-family" => property!(FontFamily, FontFamily),
      "font-style" => property!(FontStyle, FontStyle),
      "font-variant-caps" => property!(FontVariantCaps, FontVariantCaps),
      "line-height" => property!(LineHeight, LineHeight),
      "font" => property!(Font, Font),
      "transition-property" => property!(TransitionProperty, CustomIdent, true),
      "transition-duration" => property!(TransitionDuration, Time, true),
      "transition-delay" => property!(TransitionDelay, Time, true),
      "transition-timing-function" => property!(TransitionTimingFunction, EasingFunction, true),
      "transition" => property!(Transition, Transition, true),
      "animation-name" => property!(AnimationName, AnimationName, true),
      "animation-duration" => property!(AnimationDuration, Time, true),
      "animation-timing-function" => property!(AnimationTimingFunction, EasingFunction, true),
      "animation-iteration-count" => property!(AnimationIterationCount, AnimationIterationCount, true),
      "animation-direction" => property!(AnimationDirection, AnimationDirection, true),
      "animation-play-state" => property!(AnimationPlayState, AnimationPlayState, true),
      "animation-delay" => property!(AnimationDelay, Time, true),
      "animation-fill-mode" => property!(AnimationFillMode, AnimationFillMode, true),
      "animation" => property!(Animation, Animation, true),
      "transform" => property!(Transform, TransformList),
      _ => {}
    }

    input.reset(&state);
    return Ok(Property::Custom(CustomProperty::parse(name, input)?))
  }

  pub fn to_css<W>(&self, dest: &mut Printer<W>) -> std::fmt::Result where W: std::fmt::Write {
    use Property::*;

    macro_rules! property {
      ($prop: literal, $value: expr) => {{
        dest.write_str($prop)?;
        dest.delim(':', false)?;
        $value.to_css(dest)
      }};
      ($prop: literal, $value: expr, $multi: expr) => {{
        dest.write_str($prop)?;
        dest.delim(':', false)?;
        let len = $value.len();
        for (idx, val) in $value.iter().enumerate() {
          val.to_css(dest)?;
          if idx < len - 1 {
            dest.delim(',', false)?;
          }
        }
        Ok(())
      }};
    }

    match self {
      BackgroundColor(color) => property!("background-color", color),
      BackgroundImage(image) => property!("background-image", image, true),
      BackgroundPositionX(val) => property!("background-position-x", val, true),
      BackgroundPositionY(val) => property!("background-position-y", val, true),
      BackgroundPosition(val) => property!("background-position", val, true),
      BackgroundRepeat(val) => property!("background-repeat", val, true),
      BackgroundSize(val) => property!("background-size", val, true),
      BackgroundAttachment(val) => property!("background-attachment", val, true),
      BackgroundOrigin(val) => property!("background-origin", val, true),
      BackgroundClip(val) => property!("background-clip", val, true),
      Background(val) => property!("background", val, true),
      BoxShadow(val) => property!("box-shadow", val, true),
      Opacity(val) => property!("opacity", val),
      Color(color) => property!("color", color),
      Width(val) => property!("width", val),
      Height(val) => property!("height", val),
      MinWidth(val) => property!("min-width", val),
      MinHeight(val) => property!("min-height", val),
      MaxWidth(val) => property!("max-width", val),
      MaxHeight(val) => property!("max-height", val),
      BlockSize(val) => property!("block-size", val),
      InlineSize(val) => property!("inline-size", val),
      MinBlockSize(val) => property!("min-block-size", val),
      MinInlineSize(val) => property!("min-inline-size", val),
      MaxBlockSize(val) => property!("max-block-size", val),
      MaxInlineSize(val) => property!("max-inline-size", val),
      Top(val) => property!("top", val),
      Bottom(val) => property!("bottom", val),
      Left(val) => property!("left", val),
      Right(val) => property!("right", val),
      InsetBlockStart(val) => property!("inset-block-start", val),
      InsetBlockEnd(val) => property!("inset-block-end", val),
      InsetInlineStart(val) => property!("inset-inline-start", val),
      InsetInlineEnd(val) => property!("inset-inline-end", val),
      BorderTopColor(val) => property!("border-top-color", val),
      BorderBottomColor(val) => property!("border-bottom-color", val),
      BorderLeftColor(val) => property!("border-left-color", val),
      BorderRightColor(val) => property!("border-right-color", val),
      BorderBlockStartColor(val) => property!("border-block-start-color", val),
      BorderBlockEndColor(val) => property!("border-block-end-color", val),
      BorderInlineStartColor(val) => property!("border-inline-start-color", val),
      BorderInlineEndColor(val) => property!("border-inline-end-color", val),
      BorderTopStyle(val) => property!("border-top-style", val),
      BorderBottomStyle(val) => property!("border-bottom-style", val),
      BorderLeftStyle(val) => property!("border-left-style", val),
      BorderRightStyle(val) => property!("border-right-style", val),
      BorderBlockStartStyle(val) => property!("border-block-start-style", val),
      BorderBlockEndStyle(val) => property!("border-block-end-style", val),
      BorderInlineStartStyle(val) => property!("border-inline-start-style", val),
      BorderInlineEndStyle(val) => property!("border-inline-end-style", val),
      BorderTopWidth(val) => property!("border-top-width", val),
      BorderBottomWidth(val) => property!("border-bottom-width", val),
      BorderLeftWidth(val) => property!("border-left-width", val),
      BorderRightWidth(val) => property!("border-right-width", val),
      BorderBlockStartWidth(val) => property!("border-block-start-width", val),
      BorderBlockEndWidth(val) => property!("border-block-end-width", val),
      BorderInlineStartWidth(val) => property!("border-inline-start-width", val),
      BorderInlineEndWidth(val) => property!("border-inline-end-width", val),
      BorderColor(val) => property!("border-color", val),
      BorderStyle(val) => property!("border-style", val),
      BorderWidth(val) => property!("border-width", val),
      BorderBlockColor(val) => property!("border-block-color", val),
      BorderBlockStyle(val) => property!("border-block-style", val),
      BorderBlockWidth(val) => property!("border-block-width", val),
      BorderInlineColor(val) => property!("border-inline-color", val),
      BorderInlineStyle(val) => property!("border-inline-style", val),
      BorderInlineWidth(val) => property!("border-inline-width", val),
      Border(val) => property!("border", val),
      BorderTop(val) => property!("border-top", val),
      BorderBottom(val) => property!("border-bottom", val),
      BorderLeft(val) => property!("border-left", val),
      BorderRight(val) => property!("border-right", val),
      BorderBlock(val) => property!("border-block", val),
      BorderBlockStart(val) => property!("border-block-start", val),
      BorderBlockEnd(val) => property!("border-block-end", val),
      BorderInline(val) => property!("border-inline", val),
      BorderInlineStart(val) => property!("border-inline-start", val),
      BorderInlineEnd(val) => property!("border-inline-end", val),
      BorderImageSource(val) => property!("border-image-source", val),
      BorderImageOutset(val) => property!("border-image-outset", val),
      BorderImageRepeat(val) => property!("border-image-repeat", val),
      BorderImageWidth(val) => property!("border-image-width", val),
      BorderImageSlice(val) => property!("border-image-slice", val),
      BorderImage(val) => property!("border-image", val),
      BorderTopLeftRadius(val) => property!("border-top-left-radius", val),
      BorderTopRightRadius(val) => property!("border-top-right-radius", val),
      BorderBottomLeftRadius(val) => property!("border-bottom-left-radius", val),
      BorderBottomRightRadius(val) => property!("border-bottom-right-radius", val),
      BorderStartStartRadius(val) => property!("border-start-start-radius", val),
      BorderStartEndRadius(val) => property!("border-start-end-radius", val),
      BorderEndStartRadius(val) => property!("border-end-start-radius", val),
      BorderEndEndRadius(val) => property!("border-end-end-radius", val),
      BorderRadius(val) => property!("border-radius", val),
      Outline(val) => property!("outline", val),
      OutlineColor(val) => property!("outline-color", val),
      OutlineStyle(val) => property!("outline-style", val),
      OutlineWidth(val) => property!("outline-width", val),
      FlexDirection(val) => property!("flex-direction", val),
      FlexWrap(val) => property!("flex-wrap", val),
      FlexFlow(val) => property!("flex-flow", val),
      FlexGrow(val) => property!("flex-grow", val),
      FlexShrink(val) => property!("flex-shrink", val),
      FlexBasis(val) => property!("flex-basis", val),
      Flex(val) => property!("flex", val),
      AlignContent(val) => property!("align-content", val),
      JustifyContent(val) => property!("justify-content", val),
      PlaceContent(val) => property!("place-content", val),
      AlignSelf(val) => property!("align-self", val),
      JustifySelf(val) => property!("justify-self", val),
      PlaceSelf(val) => property!("place-self", val),
      AlignItems(val) => property!("align-items", val),
      JustifyItems(val) => property!("justify-items", val),
      PlaceItems(val) => property!("place-items", val),
      RowGap(val) => property!("row-gap", val),
      ColumnGap(val) => property!("column-gap", val),
      Gap(val) => property!("gap", val),
      MarginLeft(val) => property!("margin-left", val),
      MarginRight(val) => property!("margin-right", val),
      MarginTop(val) => property!("margin-top", val),
      MarginBottom(val) => property!("margin-bottom", val),
      MarginBlockStart(val) => property!("margin-block-start", val),
      MarginBlockEnd(val) => property!("margin-block-end", val),
      MarginInlineStart(val) => property!("margin-inline-start", val),
      MarginInlineEnd(val) => property!("margin-inline-end", val),
      MarginBlock(val) => property!("margin-block", val),
      MarginInline(val) => property!("margin-inline", val),
      Margin(val) => property!("margin", val),
      PaddingLeft(val) => property!("padding-left", val),
      PaddingRight(val) => property!("padding-right", val),
      PaddingTop(val) => property!("padding-top", val),
      PaddingBottom(val) => property!("padding-bottom", val),
      PaddingBlockStart(val) => property!("padding-block-start", val),
      PaddingBlockEnd(val) => property!("padding-block-end", val),
      PaddingInlineStart(val) => property!("padding-inline-start", val),
      PaddingInlineEnd(val) => property!("padding-inline-end", val),
      PaddingBlock(val) => property!("padding-block", val),
      PaddingInline(val) => property!("padding-inline", val),
      Padding(val) => property!("padding", val),
      ScrollMarginLeft(val) => property!("scroll-margin-left", val),
      ScrollMarginRight(val) => property!("scroll-margin-right", val),
      ScrollMarginTop(val) => property!("scroll-margin-top", val),
      ScrollMarginBottom(val) => property!("scroll-margin-bottom", val),
      ScrollMarginBlockStart(val) => property!("scroll-margin-block-start", val),
      ScrollMarginBlockEnd(val) => property!("scroll-margin-block-end", val),
      ScrollMarginInlineStart(val) => property!("scroll-margin-inline-start", val),
      ScrollMarginInlineEnd(val) => property!("scroll-margin-inline-end", val),
      ScrollMarginBlock(val) => property!("scroll-margin-block", val),
      ScrollMarginInline(val) => property!("scroll-margin-inline", val),
      ScrollMargin(val) => property!("scroll-margin", val),
      ScrollPaddingLeft(val) => property!("scroll-padding-left", val),
      ScrollPaddingRight(val) => property!("scroll-padding-right", val),
      ScrollPaddingTop(val) => property!("scroll-padding-top", val),
      ScrollPaddingBottom(val) => property!("scroll-padding-bottom", val),
      ScrollPaddingBlockStart(val) => property!("scroll-padding-block-start", val),
      ScrollPaddingBlockEnd(val) => property!("scroll-padding-block-end", val),
      ScrollPaddingInlineStart(val) => property!("scroll-padding-inline-start", val),
      ScrollPaddingInlineEnd(val) => property!("scroll-padding-inline-end", val),
      ScrollPaddingBlock(val) => property!("scroll-padding-block", val),
      ScrollPaddingInline(val) => property!("scroll-padding-inline", val),
      ScrollPadding(val) => property!("scroll-padding", val),
      FontWeight(val) => property!("font-weight", val),
      FontSize(val) => property!("font-size", val),
      FontStretch(val) => property!("font-stretch", val),
      FontFamily(val) => property!("font-family", val, true),
      FontStyle(val) => property!("font-style", val),
      FontVariantCaps(val) => property!("font-variant-caps", val),
      LineHeight(val) => property!("line-height", val),
      Font(val) => property!("font", val),
      TransitionProperty(val) => property!("transition-property", val, true),
      TransitionDuration(val) => property!("transition-duration", val, true),
      TransitionDelay(val) => property!("transition-delay", val, true),
      TransitionTimingFunction(val) => property!("transition-timing-function", val, true),
      Transition(val) => property!("transition", val, true),
      AnimationName(val) => property!("animation-name", val, true),
      AnimationDuration(val) => property!("animation-duration", val, true),
      AnimationTimingFunction(val) => property!("animation-timing-function", val, true),
      AnimationIterationCount(val) => property!("animation-iteration-count", val, true),
      AnimationDirection(val) => property!("animation-direction", val, true),
      AnimationPlayState(val) => property!("animation-play-state", val, true),
      AnimationDelay(val) => property!("animation-delay", val, true),
      AnimationFillMode(val) => property!("animation-fill-mode", val, true),
      Animation(val) => property!("animation", val, true),
      Transform(val) => property!("transform", val),
      Custom(custom) => {
        dest.write_str(custom.name.as_ref())?;
        dest.delim(':', false)?;
        dest.write_str(custom.value.as_ref())
      }
    }
  }
}
