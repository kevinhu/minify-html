use ::phf::{phf_set, Set};

static LAYOUT_TAGS: Set<&'static str> = phf_set! {
    // Sectioning tags.
	"article",
	"aside",
	"nav",
	"section",
	// Other tags.
	"blockquote",
	"body",
	"colgroup",
	"datalist",
	"dialog",
	"div",
	"dl",
	"fieldset",
	"figure",
	"footer",
	"form",
	"head",
	"header",
	"hgroup",
	"html",
	"main",
	"map",
	"menu",
	"nav",
	"ol",
	"optgroup",
	"picture",
	"section",
	"select",
	"table",
	"tbody",
	"tfoot",
	"thead",
	"tr",
	"ul",
};
