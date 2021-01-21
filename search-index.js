var searchIndex = JSON.parse('{\
"cli_table":{"doc":"Rust crate for printing tables on command line.","i":[[0,"format","cli_table","Utilities for formatting of a table",null,null],[4,"Align","cli_table::format","Used to vertically align contents of a cell",null,null],[13,"Top","","Aligns contents to top",0,null],[13,"Bottom","","Aligns contents to bottom",0,null],[13,"Center","","Aligns contents to center",0,null],[4,"Justify","","Used to horizontally justify contents of a cell",null,null],[13,"Left","","Justifies contents to left",1,null],[13,"Right","","Justifies contents to right",1,null],[13,"Center","","Justifies contents to center",1,null],[3,"Padding","","Used to add padding to the contents of a cell",null,null],[3,"PaddingBuilder","","Builder for padding",null,null],[3,"Border","","Borders of a table",null,null],[3,"BorderBuilder","","Builder for border",null,null],[3,"HorizontalLine","","A horizontal line in a table (border or row separator)",null,null],[3,"Separator","","Inner (column/row) separators of a table",null,null],[3,"SeparatorBuilder","","Builder for separator",null,null],[3,"VerticalLine","","A vertical line in a table (border or column separator)",null,null],[4,"Color","cli_table","The set of available colors for the terminal …",null,null],[13,"Black","","",2,null],[13,"Blue","","",2,null],[13,"Green","","",2,null],[13,"Red","","",2,null],[13,"Cyan","","",2,null],[13,"Magenta","","",2,null],[13,"Yellow","","",2,null],[13,"White","","",2,null],[13,"Ansi256","","",2,null],[13,"Rgb","","",2,null],[4,"ColorChoice","","ColorChoice represents the color preferences of an end …",null,null],[13,"Always","","Try very hard to emit colors. This includes emitting ANSI …",3,null],[13,"AlwaysAnsi","","AlwaysAnsi is like Always, except it never tries to use …",3,null],[13,"Auto","","Try to use colors, but don\'t force the issue. If the …",3,null],[13,"Never","","Never emit colors.",3,null],[24,"Table","","Derive macro to implementing <code>cli_table</code> traits",null,null],[8,"Cell","","Trait to convert raw types into cells",null,null],[10,"cell","","Converts raw type to cell of a table",4,[[],["cellstruct",3]]],[3,"CellStruct","","Concrete cell of a table",null,null],[8,"Row","","Trait to convert raw types into rows",null,null],[10,"row","","Converts raw type to rows of a table",5,[[],["rowstruct",3]]],[3,"RowStruct","","Concrete row of a table",null,null],[8,"Style","","Trait for modifying style of table and cells",null,null],[10,"foreground_color","","Used to set foreground color",6,[[["color",4],["option",4]]]],[10,"background_color","","Used to set background color",6,[[["color",4],["option",4]]]],[10,"bold","","Used to set contents to be bold",6,[[]]],[10,"underline","","Used to set contents to be underlined",6,[[]]],[10,"italic","","Used to set contents to be italic",6,[[]]],[10,"intense","","Used to set high intensity version of a color specified",6,[[]]],[10,"dimmed","","Used to set contents to be dimmed",6,[[]]],[8,"Table","","Trait to convert raw type into table",null,null],[10,"table","","Converts raw type to a table",7,[[],["tablestruct",3]]],[3,"TableStruct","","Struct for building a table on command line",null,null],[8,"Title","","Trait for getting title row of a struct",null,null],[10,"title","","Returns title row of a struct",8,[[],["rowstruct",3]]],[8,"WithTitle","","Trait for creating a table with titles at the top",null,null],[10,"with_title","","Creates a table with title at the top",9,[[],["tablestruct",3]]],[5,"print_stdout","","Prints a table to <code>stdout</code>",null,[[["table",8]],["result",6]]],[5,"print_stderr","","Prints a table to <code>stderr</code>",null,[[["table",8]],["result",6]]],[11,"from","","",10,[[]]],[11,"into","","",10,[[]]],[11,"borrow","","",10,[[]]],[11,"borrow_mut","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"try_into","","",10,[[],["result",4]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"from","cli_table::format","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_owned","","",1,[[]]],[11,"clone_into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_owned","","",0,[[]]],[11,"clone_into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",11,[[]]],[11,"into","","",11,[[]]],[11,"to_owned","","",11,[[]]],[11,"clone_into","","",11,[[]]],[11,"borrow","","",11,[[]]],[11,"borrow_mut","","",11,[[]]],[11,"try_from","","",11,[[],["result",4]]],[11,"try_into","","",11,[[],["result",4]]],[11,"type_id","","",11,[[],["typeid",3]]],[11,"from","","",12,[[]]],[11,"into","","",12,[[]]],[11,"borrow","","",12,[[]]],[11,"borrow_mut","","",12,[[]]],[11,"try_from","","",12,[[],["result",4]]],[11,"try_into","","",12,[[],["result",4]]],[11,"type_id","","",12,[[],["typeid",3]]],[11,"from","cli_table","",13,[[]]],[11,"into","","",13,[[]]],[11,"borrow","","",13,[[]]],[11,"borrow_mut","","",13,[[]]],[11,"try_from","","",13,[[],["result",4]]],[11,"try_into","","",13,[[],["result",4]]],[11,"type_id","","",13,[[],["typeid",3]]],[11,"from","","",14,[[]]],[11,"into","","",14,[[]]],[11,"borrow","","",14,[[]]],[11,"borrow_mut","","",14,[[]]],[11,"try_from","","",14,[[],["result",4]]],[11,"try_into","","",14,[[],["result",4]]],[11,"type_id","","",14,[[],["typeid",3]]],[11,"from","cli_table::format","",15,[[]]],[11,"into","","",15,[[]]],[11,"to_owned","","",15,[[]]],[11,"clone_into","","",15,[[]]],[11,"borrow","","",15,[[]]],[11,"borrow_mut","","",15,[[]]],[11,"try_from","","",15,[[],["result",4]]],[11,"try_into","","",15,[[],["result",4]]],[11,"type_id","","",15,[[],["typeid",3]]],[11,"from","","",16,[[]]],[11,"into","","",16,[[]]],[11,"to_owned","","",16,[[]]],[11,"clone_into","","",16,[[]]],[11,"borrow","","",16,[[]]],[11,"borrow_mut","","",16,[[]]],[11,"try_from","","",16,[[],["result",4]]],[11,"try_into","","",16,[[],["result",4]]],[11,"type_id","","",16,[[],["typeid",3]]],[11,"from","","",17,[[]]],[11,"into","","",17,[[]]],[11,"to_owned","","",17,[[]]],[11,"clone_into","","",17,[[]]],[11,"borrow","","",17,[[]]],[11,"borrow_mut","","",17,[[]]],[11,"try_from","","",17,[[],["result",4]]],[11,"try_into","","",17,[[],["result",4]]],[11,"type_id","","",17,[[],["typeid",3]]],[11,"from","","",18,[[]]],[11,"into","","",18,[[]]],[11,"to_owned","","",18,[[]]],[11,"clone_into","","",18,[[]]],[11,"borrow","","",18,[[]]],[11,"borrow_mut","","",18,[[]]],[11,"try_from","","",18,[[],["result",4]]],[11,"try_into","","",18,[[],["result",4]]],[11,"type_id","","",18,[[],["typeid",3]]],[11,"from","","",19,[[]]],[11,"into","","",19,[[]]],[11,"to_owned","","",19,[[]]],[11,"clone_into","","",19,[[]]],[11,"borrow","","",19,[[]]],[11,"borrow_mut","","",19,[[]]],[11,"try_from","","",19,[[],["result",4]]],[11,"try_into","","",19,[[],["result",4]]],[11,"type_id","","",19,[[],["typeid",3]]],[11,"from","","",20,[[]]],[11,"into","","",20,[[]]],[11,"borrow","","",20,[[]]],[11,"borrow_mut","","",20,[[]]],[11,"try_from","","",20,[[],["result",4]]],[11,"try_into","","",20,[[],["result",4]]],[11,"type_id","","",20,[[],["typeid",3]]],[11,"from","cli_table","",2,[[]]],[11,"into","","",2,[[]]],[11,"to_owned","","",2,[[]]],[11,"clone_into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"to_owned","","",3,[[]]],[11,"clone_into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"clone","","",3,[[],["colorchoice",4]]],[11,"clone","","",2,[[],["color",4]]],[11,"fmt","","",2,[[["formatter",3]],[["error",3],["result",4]]]],[11,"fmt","","",3,[[["formatter",3]],[["error",3],["result",4]]]],[11,"eq","","",2,[[["color",4]]]],[11,"ne","","",2,[[["color",4]]]],[11,"eq","","",3,[[["colorchoice",4]]]],[11,"from_str","","",2,[[],[["color",4],["result",4],["parsecolorerror",3]]]],[11,"cell","","",10,[[],["cellstruct",3]]],[11,"row","","",13,[[],["rowstruct",3]]],[11,"foreground_color","","",10,[[["color",4],["option",4]]]],[11,"background_color","","",10,[[["color",4],["option",4]]]],[11,"bold","","",10,[[]]],[11,"underline","","",10,[[]]],[11,"italic","","",10,[[]]],[11,"intense","","",10,[[]]],[11,"dimmed","","",10,[[]]],[11,"foreground_color","","",14,[[["color",4],["option",4]]]],[11,"background_color","","",14,[[["color",4],["option",4]]]],[11,"bold","","",14,[[]]],[11,"underline","","",14,[[]]],[11,"italic","","",14,[[]]],[11,"intense","","",14,[[]]],[11,"dimmed","","",14,[[]]],[11,"table","","",14,[[],["tablestruct",3]]],[11,"clone","cli_table::format","",1,[[],["justify",4]]],[11,"clone","","",0,[[],["align",4]]],[11,"clone","","",11,[[],["padding",3]]],[11,"clone","","",15,[[],["verticalline",3]]],[11,"clone","","",16,[[],["horizontalline",3]]],[11,"clone","","",17,[[],["border",3]]],[11,"clone","","",18,[[],["borderbuilder",3]]],[11,"clone","","",19,[[],["separator",3]]],[11,"default","","",1,[[]]],[11,"default","","",0,[[]]],[11,"default","","",11,[[],["padding",3]]],[11,"default","","",12,[[],["paddingbuilder",3]]],[11,"default","","",15,[[]]],[11,"default","","",16,[[]]],[11,"default","","",17,[[]]],[11,"default","","",19,[[]]],[11,"eq","","",15,[[["verticalline",3]]]],[11,"ne","","",15,[[["verticalline",3]]]],[11,"eq","","",16,[[["horizontalline",3]]]],[11,"ne","","",16,[[["horizontalline",3]]]],[11,"eq","","",17,[[["border",3]]]],[11,"ne","","",17,[[["border",3]]]],[11,"eq","","",18,[[["borderbuilder",3]]]],[11,"ne","","",18,[[["borderbuilder",3]]]],[11,"eq","","",19,[[["separator",3]]]],[11,"ne","","",19,[[["separator",3]]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"fmt","","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",11,[[["formatter",3]],["result",6]]],[11,"fmt","","",12,[[["formatter",3]],["result",6]]],[11,"fmt","","",15,[[["formatter",3]],["result",6]]],[11,"fmt","","",16,[[["formatter",3]],["result",6]]],[11,"fmt","","",17,[[["formatter",3]],["result",6]]],[11,"fmt","","",18,[[["formatter",3]],["result",6]]],[11,"fmt","","",19,[[["formatter",3]],["result",6]]],[11,"fmt","","",20,[[["formatter",3]],["result",6]]],[11,"justify","cli_table","Used to horizontally justify contents of a cell",10,[[["justify",4]],["cellstruct",3]]],[11,"align","","Used to vertically align the contents of a cell",10,[[["align",4]],["cellstruct",3]]],[11,"padding","","Used to add padding to the contents of a cell",10,[[["padding",3]],["cellstruct",3]]],[11,"builder","cli_table::format","Creates a new builder for padding",11,[[],["paddingbuilder",3]]],[11,"left","","Sets left padding of a cell",12,[[]]],[11,"right","","Sets right padding of a cell",12,[[]]],[11,"top","","Sets top padding of a cell",12,[[]]],[11,"bottom","","Sets bottom padding of a cell",12,[[]]],[11,"build","","Build padding",12,[[],["padding",3]]],[11,"title","cli_table","Used to add a title row of a table",14,[[["row",8]]]],[11,"border","","Used to set border of a table",14,[[["border",3]]]],[11,"separator","","Used to set column/row separators of a table",14,[[["separator",3]]]],[11,"color_choice","","Used to set the color preferences for printing the table",14,[[["colorchoice",4]]]],[11,"new","cli_table::format","Creates a new instance of vertical line",15,[[]]],[11,"new","","Creates a new instance of horizontal line",16,[[]]],[11,"builder","","Creates a new builder for border",17,[[],["borderbuilder",3]]],[11,"top","","Set top border of a table",18,[[["horizontalline",3]]]],[11,"bottom","","Set bottom border of a table",18,[[["horizontalline",3]]]],[11,"left","","Set left border of a table",18,[[["verticalline",3]]]],[11,"right","","Set right border of a table",18,[[["verticalline",3]]]],[11,"build","","Build border",18,[[],["border",3]]],[11,"builder","","Creates a new builder for separator",19,[[],["separatorbuilder",3]]],[11,"column","","Set column separators of a table",20,[[["option",4],["verticalline",3]]]],[11,"row","","Set column separators of a table",20,[[["option",4],["horizontalline",3]]]],[11,"title","","Set title of a table",20,[[["option",4],["horizontalline",3]]]],[11,"build","","Build separator",20,[[],["separator",3]]]],"p":[[4,"Align"],[4,"Justify"],[4,"Color"],[4,"ColorChoice"],[8,"Cell"],[8,"Row"],[8,"Style"],[8,"Table"],[8,"Title"],[8,"WithTitle"],[3,"CellStruct"],[3,"Padding"],[3,"PaddingBuilder"],[3,"RowStruct"],[3,"TableStruct"],[3,"VerticalLine"],[3,"HorizontalLine"],[3,"Border"],[3,"BorderBuilder"],[3,"Separator"],[3,"SeparatorBuilder"]]},\
"cli_table_derive":{"doc":"Rust crate for printing tables on command line.","i":[[24,"Table","cli_table_derive","Derive macro to implementing <code>cli_table</code> traits",null,null]],"p":[]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);