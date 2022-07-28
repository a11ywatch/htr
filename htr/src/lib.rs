#![warn(missing_docs)]

//! HTML to react transformer
//!
//! htr is a fast html to react
//! transformer that uses btrees to search and replace
//! html properties to the react equivalent.
//!
//! # How to use htr
//!
//! There are two ways to use htr:
//!
//! - **Convert props React** transform html string to a react string.
//!   - [`convert_props_react`] is used to transform :blocking.
//! - **Convert to React**  Lets you transform the html to a react component.
//!   - [`convert_to_react`] is used to transform :blocking.
//!
//! [`convert_props_react`]: htr/fn.convert_props_react.html
//! [`convert_to_react`]: htr/fn.convert_to_react.html
//! 
//! # Basic usage
//!
//! First, you will need to add `htr` to your `Cargo.toml`.
//!
//! Next, add your html to one of the transform methods to get your react,
//! output.

#[macro_use]
extern crate lazy_static;

extern crate convert_case;

use convert_case::{Case, Casing};
use std::collections::BTreeMap;

/// convert props to react
pub fn convert_props_react(ctx: String) -> String {
    let mut context = ctx.clone();

    lazy_static! {
        /// html static list of properties that convert to camel-case [https://reactjs.org/docs/dom-elements.html]
        static ref HTML_PROPS: BTreeMap<&'static str, &'static str> = BTreeMap::from([
            ("acceptcharset", "acceptCharset"),
            ("accesskey", "accessKey"),
            ("allowfullscreen", "allowFullScreen"),
            ("allowtransparency", "allowTransparency"),
            ("autocomplete", "autoComplete"),
            ("autofocus", "autoFocus"),
            ("autoplay", "autoPlay"),
            ("cellpadding", "cellPadding"),
            ("cellspacing", "cellSpacing"),
            ("charset", "charSet"),
            ("class", "className"), // special html
            ("classid", "classID"),
            ("classname", "className"),
            ("colspan", "colSpan"),
            ("contenteditable", "contentEditable"),
            ("contextmenu", "contextMenu"),
            ("crossorigin", "crossOrigin"),
            ("datetime", "dateTime"),
            ("enctype", "encType"),
            ("for", "htmlFor"), // special html
            ("formaction", "formAction"),
            ("formenctype", "formEncType"),
            ("formmethod", "formMethod"),
            ("formnovalidate", "formNoValidate"),
            ("formtarget", "formTarget"),
            ("frameborder", "frameBorder"),
            ("hreflang", "hrefLang"),
            ("htmlfor", "htmlFor"),
            ("httpequiv", "httpEquiv"),
            ("inputmode", "inputMode"),
            ("keyparams", "keyParams"),
            ("keytype", "keyType"),
            ("marginheight", "marginHeight"),
            ("marginwidth", "marginWidth"),
            ("maxlength", "maxLength"),
            ("mediagroup", "mediaGroup"),
            ("minlength", "minLength"),
            ("novalidate", "noValidate"),
            ("radiogroup", "radioGroup"),
            ("readonly", "readOnly"),
            ("rowspan", "rowSpan"),
            ("spellcheck", "spellCheck"),
            ("srcdoc", "srcDoc"),
            ("srclang", "srcLang"),
            ("srcset", "srcSet"),
            ("tabindex", "tabIndex"),
            ("usemap", "useMap"),
            // svg
            ("accentheight", "accentHeight"),
            ("alignmentbaseline", "alignmentBaseline"),
            ("allowreorder", "allowReorder"),
            ("arabicform", "arabicForm"),
            ("attributename", "attributeName"),
            ("attributetype", "attributeType"),
            ("autoreverse", "autoReverse"),
            ("basefrequency", "baseFrequency"),
            ("baseprofile", "baseProfile"),
            ("baselineshift", "baselineShift"),
            ("calcmode", "calcMode"),
            ("capheight", "capHeight"),
            ("clippath", "clipPath"),
            ("clippathunits", "clipPathUnits"),
            ("cliprule", "clipRule"),
            ("colorinterpolation", "colorInterpolation"),
            ("colorinterpolationfilters", "colorInterpolationFilters"),
            ("colorprofile", "colorProfile"),
            ("colorrendering", "colorRendering"),
            ("contentscripttype", "contentScriptType"),
            ("contentstyletype", "contentStyleType"),
            ("diffuseconstant", "diffuseConstant"),
            ("dominantbaseline", "dominantBaseline"),
            ("edgemode", "edgeMode"),
            ("enablebackground", "enableBackground"),
            ("externalresourcesrequired", "externalResourcesRequired"),
            ("fillopacity", "fillOpacity"),
            ("fillrule", "fillRule"),
            ("filterres", "filterRes"),
            ("filterunits", "filterUnits"),
            ("floodcolor", "floodColor"),
            ("floodopacity", "floodOpacity"),
            ("fontfamily", "fontFamily"),
            ("fontsize", "fontSize"),
            ("fontsizeadjust", "fontSizeAdjust"),
            ("fontstretch", "fontStretch"),
            ("fontstyle", "fontStyle"),
            ("fontvariant", "fontVariant"),
            ("fontweight", "fontWeight"),
            ("glyphname", "glyphName"),
            ("glyphorientationhorizontal", "glyphOrientationHorizontal"),
            ("glyphorientationvertical", "glyphOrientationVertical"),
            ("glyphref", "glyphRef"),
            ("gradienttransform", "gradientTransform"),
            ("gradientunits", "gradientUnits"),
            ("horizadvx", "horizAdvX"),
            ("horizoriginx", "horizOriginX"),
            ("imagerendering", "imageRendering"),
            ("kernelmatrix", "kernelMatrix"),
            ("kernelunitlength", "kernelUnitLength"),
            ("keypoints", "keyPoints"),
            ("keysplines", "keySplines"),
            ("keytimes", "keyTimes"),
            ("lengthadjust", "lengthAdjust"),
            ("letterspacing", "letterSpacing"),
            ("lightingcolor", "lightingColor"),
            ("limitingconeangle", "limitingConeAngle"),
            ("markerend", "markerEnd"),
            ("markerheight", "markerHeight"),
            ("markermid", "markerMid"),
            ("markerstart", "markerStart"),
            ("markerunits", "markerUnits"),
            ("markerwidth", "markerWidth"),
            ("maskcontentunits", "maskContentUnits"),
            ("maskunits", "maskUnits"),
            ("numoctaves", "numOctaves"),
            ("overlineposition", "overlinePosition"),
            ("overlinethickness", "overlineThickness"),
            ("paintorder", "paintOrder"),
            ("pathlength", "pathLength"),
            ("patterncontentunits", "patternContentUnits"),
            ("patterntransform", "patternTransform"),
            ("patternunits", "patternUnits"),
            ("pointerevents", "pointerEvents"),
            ("pointsatx", "pointsAtX"),
            ("pointsaty", "pointsAtY"),
            ("pointsatz", "pointsAtZ"),
            ("preservealpha", "preserveAlpha"),
            ("preserveaspectratio", "preserveAspectRatio"),
            ("primitiveunits", "primitiveUnits"),
            ("refx", "refX"),
            ("refy", "refY"),
            ("renderingintent", "renderingIntent"),
            ("repeatcount", "repeatCount"),
            ("repeatdur", "repeatDur"),
            ("requiredextensions", "requiredExtensions"),
            ("requiredfeatures", "requiredFeatures"),
            ("shaperendering", "shapeRendering"),
            ("specularconstant", "specularConstant"),
            ("specularexponent", "specularExponent"),
            ("spreadmethod", "spreadMethod"),
            ("startoffset", "startOffset"),
            ("stddeviation", "stdDeviation"),
            ("stitchtiles", "stitchTiles"),
            ("stopcolor", "stopColor"),
            ("stopopacity", "stopOpacity"),
            ("strikethroughposition", "strikethroughPosition"),
            ("strikethroughthickness", "strikethroughThickness"),
            ("strokedasharray", "strokeDasharray"),
            ("strokedashoffset", "strokeDashoffset"),
            ("strokelinecap", "strokeLinecap"),
            ("strokelinejoin", "strokeLinejoin"),
            ("strokemiterlimit", "strokeMiterlimit"),
            ("strokeopacity", "strokeOpacity"),
            ("strokewidth", "strokeWidth"),
            ("surfacescale", "surfaceScale"),
            ("systemlanguage", "systemLanguage"),
            ("tablevalues", "tableValues"),
            ("targetx", "targetX"),
            ("targety", "targetY"),
            ("textanchor", "textAnchor"),
            ("textdecoration", "textDecoration"),
            ("textlength", "textLength"),
            ("textrendering", "textRendering"),
            ("underlineposition", "underlinePosition"),
            ("underlinethickness", "underlineThickness"),
            ("unicodebidi", "unicodeBidi"),
            ("unicoderange", "unicodeRange"),
            ("unitsperem", "unitsPerEm"),
            ("valphabetic", "vAlphabetic"),
            ("vhanging", "vHanging"),
            ("videographic", "vIdeographic"),
            ("vmathematical", "vMathematical"),
            ("vectoreffect", "vectorEffect"),
            ("vertadvy", "vertAdvY"),
            ("vertoriginx", "vertOriginX"),
            ("vertoriginy", "vertOriginY"),
            ("viewbox", "viewBox"),
            ("viewtarget", "viewTarget"),
            ("wordspacing", "wordSpacing"),
            ("writingmode", "writingMode"),
            ("xchannelselector", "xChannelSelector"),
            ("xheight", "xHeight"),
            ("xlinkactuate", "xlinkActuate"),
            ("xlinkarcrole", "xlinkArcrole"),
            ("xlinkhref", "xlinkHref"),
            ("xlinkrole", "xlinkRole"),
            ("xlinkshow", "xlinkShow"),
            ("xlinktitle", "xlinkTitle"),
            ("xlinktype", "xlinkType"),
            ("xmlnsxlink", "xmlnsXlink"),
            ("xmlbase", "xmlBase"),
            ("xmllang", "xmlLang"),
            ("xmlspace", "xmlSpace"),
            ("ychannelselector", "yChannelSelector"),
            ("zoomandpan", "zoomAndPan"),
            // events
            ("onabort", "onAbort"),
            ("onanimationend", "onAnimationEnd"),
            ("onanimationiteration", "onAnimationIteration"),
            ("onanimationstart", "onAnimationStart"),
            ("onblur", "onBlur"),
            ("oncanplay", "onCanPlay"),
            ("oncanplaythrough", "onCanPlayThrough"),
            ("onchange", "onChange"),
            ("onclick", "onClick"),
            ("oncompositionend", "onCompositionEnd"),
            ("oncompositionstart", "onCompositionStart"),
            ("oncompositionupdate", "onCompositionUpdate"),
            ("oncontextmenu", "onContextMenu"),
            ("oncopy", "onCopy"),
            ("oncut", "onCut"),
            ("ondoubleclick", "onDoubleClick"),
            ("ondrag", "onDrag"),
            ("ondragend", "onDragEnd"),
            ("ondragenter", "onDragEnter"),
            ("ondragexit", "onDragExit"),
            ("ondragleave", "onDragLeave"),
            ("ondragover", "onDragOver"),
            ("ondragstart", "onDragStart"),
            ("ondrop", "onDrop"),
            ("ondurationchange", "onDurationChange"),
            ("onemptied", "onEmptied"),
            ("onencrypted", "onEncrypted"),
            ("onended", "onEnded"),
            ("onerror", "onError"),
            ("onfocus", "onFocus"),
            ("oninput", "onInput"),
            ("onkeydown", "onKeyDown"),
            ("onkeypress", "onKeyPress"),
            ("onkeyup", "onKeyUp"),
            ("onload", "onLoad"),
            ("onloadeddata", "onLoadedData"),
            ("onloadedmetadata", "onLoadedMetadata"),
            ("onloadstart", "onLoadStart"),
            ("onmousedown", "onMouseDown"),
            ("onmouseenter", "onMouseEnter"),
            ("onmouseleave", "onMouseLeave"),
            ("onmousemove", "onMouseMove"),
            ("onmouseout", "onMouseOut"),
            ("onmouseover", "onMouseOver"),
            ("onmouseup", "onMouseUp"),
            ("onpaste", "onPaste"),
            ("onpause", "onPause"),
            ("onplay", "onPlay"),
            ("onplaying", "onPlaying"),
            ("onprogress", "onProgress"),
            ("onratechange", "onRateChange"),
            ("onscroll", "onScroll"),
            ("onseeked", "onSeeked"),
            ("onseeking", "onSeeking"),
            ("onselect", "onSelect"),
            ("onstalled", "onStalled"),
            ("onsubmit", "onSubmit"),
            ("onsuspend", "onSuspend"),
            ("ontimeupdate", "onTimeUpdate"),
            ("ontouchcancel", "onTouchCancel"),
            ("ontouchend", "onTouchEnd"),
            ("ontouchmove", "onTouchMove"),
            ("ontouchstart", "onTouchStart"),
            ("ontransitionend", "onTransitionEnd"),
            ("onvolumechange", "onVolumeChange"),
            ("onwaiting", "onWaiting"),
            ("onwheel", "onWheel")
        ]);
    };

    let props: Vec<String> = extract_html_props(&context);

    for item in props.iter() {
        if item == "style" {
            create_style_object(&mut context);
        } else {
            let value = HTML_PROPS.get(&*item.to_owned()).unwrap_or(&"");

            if !value.is_empty() {
                let v = format!("{}=", item);
                let rp = format!("{}=", value);
                context = context.replace(&v, &rp);
            }
        }
    }

    context
}

/// extract all html props into a vector
pub fn extract_html_props(context: &String) -> Vec<String> {
    let mut props: Vec<String> = vec![];
    let mut current_prop = String::from("");
    let mut space_before_text = false;
    let mut inside_tag = false;

    // get all html props into a vec
    for c in context.chars() {
        if inside_tag {
            if c == '=' {
                space_before_text = false;
                props.push((*current_prop).to_string());
                current_prop.clear();
            }
            if space_before_text {
                current_prop.push(c);
            }
            if c == ' ' {
                space_before_text = true;
            }
        }
        if c == '<' {
            inside_tag = true;
        }
        if c == '>' {
            inside_tag = false;
        }
    }

    // sort the vec for btree linear lookup performance
    props.sort();

    props
}

/// manipulate and mutate the style properties to react
pub fn create_style_object(ctx: &mut String) -> &mut String {
    let style_matcher = if ctx.contains("style='") {
        r#"'"#
    } else {
        r#"""#
    };

    let style_start = format!(r#"style={}"#, style_matcher);
    let (style_string, start_idx, end_idx) = text_between(&ctx, &style_start, style_matcher);

    let mut current_prop = String::from("");
    let mut space_before_text = false;

    let mut style_replacer = style_string.clone();

    // get all html props into a vec
    for c in style_string.chars() {
        if space_before_text {
            current_prop.push(c);
        }
        if c == ';' {
            space_before_text = true;
            style_replacer = style_replacer.replace(";", ",");
            current_prop.clear();
        }
        if c == ':' {
            let clp = &current_prop.trim();
            let camel_style = &clp.to_case(Case::Camel);

            style_replacer = style_replacer.replace(&*clp, &camel_style);
            space_before_text = false;
            current_prop.clear();
        }
    }

    let mut space_before_text = false;
    let mut current_prop = String::from("");

    let mut style_clone = style_replacer.clone().to_owned();

    // add double quotes to react props style values
    for (i, c) in style_replacer.chars().enumerate() {
        if space_before_text && c != ',' {
            current_prop.push(c);
        }

        if space_before_text && c == ',' {
            let current = current_prop.trim();
            style_clone = style_clone.replace(&current, &format!(r#""{}""#, current).to_string());
            space_before_text = false;
        }

        if c == ':' {
            space_before_text = true;
            current_prop.clear();
        }

        if i + 1 == style_replacer.len() {
            let current = current_prop.trim();
            style_clone = style_clone.replace(&current, &format!(r#""{}""#, current).to_string());
            space_before_text = false;
        }
    }

    let style_replacer = format!("{}{}{}", "style={{", style_clone, "}}");

    ctx.replace_range(start_idx - 7..start_idx + end_idx + 1, &style_replacer);

    ctx
}

/// get the text between two strings
pub fn text_between(search_str: &String, start_str: &String, end_str: &str) -> (String, usize, usize) {
    let start_idx = {
        let start_point = search_str.find(start_str);
        start_point.unwrap() + start_str.len()
    };

    let remaining = &search_str[start_idx..];
    let end_idx = remaining.find(&end_str).unwrap_or(remaining.len());

    (remaining[..end_idx].to_string(), start_idx, end_idx)
}

/// convert props to a react component
pub fn convert_to_react(ctx: String, component_name: String) -> String {
    let react_html = convert_props_react(ctx);
    let mut react_html = react_html.trim().to_owned();
    
    // remove html tags
    if react_html.starts_with("<!DOCTYPE html>") {
        react_html = react_html.replace("<!DOCTYPE html>", "").trim().to_owned();
    }
    if react_html.starts_with("<html>") {
        react_html = react_html.replace("<html>", "");
        react_html = react_html.replace("</html>", "");
    }

    let component_name = format!(" {}", component_name.trim());

    let component = format!(
        r###"import React from "react"
    
function{}() {{
    return (
        <>
        {}
        </>
    )
}}"###,
        component_name, react_html
    );

    component
}