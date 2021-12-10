var searchIndex = JSON.parse('{\
"bittongue":{"doc":"","t":[0,0,0,8,3,13,3,3,4,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,11,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,8,3,3,8,16,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,11,11,10,11,12,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,3,3,16,3,3,3,8,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["diagnostic","lexer","source","Diagnostic","Diagnostics","Error","IntoIter","Iter","Level","Note","Warning","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone_into","cmp","default","downcast","downcast","downcast","downcast","downcast_mut","downcast_mut","downcast_mut","downcast_mut","downcast_ref","downcast_ref","downcast_ref","downcast_ref","eq","fmt","fmt","fmt","fmt","from","from","from","from","hash","into","into","into","into","into_iter","into_iter","into_iter","is","is","is","is","is_err","is_ok","iter","level","max_level","new","next","next","partial_cmp","primary_span","raise","secondary_spans","to_owned","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","Lexer","LexingError","Token","TokenKind","TokenKind","TokenStream","advance","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone_into","clone_into","clone_into","cmp","current","default","eq","fmt","fmt","fmt","from","from","from","generate_token","hash","into","into","into","is_eof","is_eof","kind","ne","new","next","partial_cmp","position","prev","rollback","source","span","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","Location","NewlineIndices","Output","Reader","SegmentIndices","Source","SourceIndex","Span","SpanContent","advance","as_ref","as_ref","as_str","as_str","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","cmp","cmp","cmp","cmp","column","content","contents","current","current_to","deref","end","eq","eq","eq","eq","eq","expand_lines","expect","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","get","get","hash","hash","hash","hash","index","index","index","into","into","into","into","into","into","into","into_iter","into_iter","is_eof","len","len","line","line_column","line_span","location","mark","marked","name","ne","ne","new","newlines","next","next","next","next_back","next_back","partial_cmp","partial_cmp","partial_cmp","partial_cmp","partial_cmp","position","position","prev","reader","rollback","segments","size_hint","size_hint","slice","source","source","source","span","span","start","to_owned","to_owned","to_owned","to_owned","to_owned","to_string","to_string","to_string","to_string","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_slice","type_id","type_id","type_id","type_id","type_id","type_id","type_id"],"q":["bittongue","","","bittongue::diagnostic","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","bittongue::lexer","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","bittongue::source","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["This module exports error-related utilities for …","","This module provides utilities to handle source code of a …","Diagnostic: a problem or note found in a source code.","A collection of diagnostics. Generic on the type of …","A hard error, cannot be ignored at all.","Owned iterator over diagnostics of the <code>Diagnostics</code> …","Borrowed iterator over diagnostics of the <code>Diagnostics</code> …","Level of a given diagnostic.","This is just a note, easily ignored.","This is a warning, should be read carefully.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns whether the source code status is NOT OK (there …","Returns whether the source code status is OK (no hard …","Creates an iterator over references of diagnostics.","Severity level of this diagnostic.","Returns the maximum level among the diagnostics in this …","Creates an empty collection of diagnostics.","","","","Yields the primary span related to the diagnostic, if any.","Raises a new diagnostic and saves it in this collection.","Yields an iterator over secondary spans related to the …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","The location in a source code.","Iterator over the newline indices of a source, in terms of …","Output of the indexing operation.","A reader of a source code, a stream.","Iterator over the segment indices of a source, in terms of …","A source code object, such as read from a file.","An index on a source code.","A span (a range) in the source code.","A type that, when displayed, shows the span contents, …","Advance the stream by the given <code>count</code> of string segments, …","","","Gets the string this span includes as a whole.","Returns the span contents as a string.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Finds the column of this location in the source code.","Creates a type that, when displayed, shows the span …","The contents of the source.","The current string segment rendered.","A string segment from current position until …","","The end location of this span.","","","","","","Expands this span in order to contain the whole lines the …","","","","","","","","","","","","","","","","","","","","Indexes the source code and returns <code>None</code> if out of bounds.","Indexes this source. It can be a single <code>usize</code> or a range …","","","","","Indexes the source code and panics if out of bounds.","Indexes the source code and panics if out of bounds.","","","","","","","","","","","Is the end of file reached?","The length of this span in string segments.","The length the source.","Finds the line of this location in the source code.","Finds the line and column (respectively) of this location …","Creates a <code>Span</code> containing the whole line this location is …","Location at the given position.","Marks the current position so it can be used to create a …","The marked position (in string segments).","The (file) name of the source.","","","Creates a new source code object given its name and its …","Iterator over the newline indices of the source, in terms …","Advances the stream by 1 and returns whether it did move.","","","","","","","","","","The string segment position in the source code.","Position in string segments that the reader is currently …","Goes back on the stream by 1 and returns whether it did …","Creates a source code reader (a stream) from this source …","Goes back on the stream by the given <code>count</code> of string …","Iterator over the segment indices of the source, in terms …","","","Slices this span to the given range.","The source code object this location refers to.","The source code this reader is reading.","The source code object this span refers to.","<code>Span</code> from the marked position up to the current position.","Returns the inner span.","The start location of this span.","","","","","","","","","","","","","","","","","","","","","","","","Slices this span to the given range. Returns <code>None</code> if the …","","","","","","",""],"i":[0,0,0,0,0,1,0,0,0,1,1,1,2,3,4,1,2,3,4,1,1,1,2,5,5,5,5,5,5,5,5,5,5,5,5,1,1,2,3,4,1,2,3,4,1,1,2,3,4,2,3,4,5,5,5,5,2,2,2,5,2,2,3,4,1,5,2,5,1,1,2,3,4,1,2,3,4,1,2,3,4,0,0,0,0,6,0,7,8,9,7,8,9,7,8,9,7,8,9,7,9,7,8,9,8,9,7,8,9,7,6,9,8,9,7,10,7,9,9,7,7,9,7,7,7,7,9,8,9,7,8,9,7,8,9,7,8,9,7,0,0,11,0,0,0,0,0,0,12,13,14,13,14,15,12,13,14,14,16,17,18,15,12,13,14,16,17,18,15,12,13,14,16,15,12,13,14,16,15,13,14,16,15,13,16,12,12,14,13,15,13,14,14,16,13,12,15,15,12,13,13,14,14,16,16,17,18,15,12,13,14,16,17,18,11,16,15,13,14,16,11,11,16,15,12,13,14,16,17,18,17,18,12,13,16,15,15,15,12,12,12,16,15,13,16,16,12,17,18,17,18,15,13,14,14,16,15,12,12,16,12,16,17,18,13,15,12,13,12,14,13,15,12,13,14,16,15,13,14,16,15,12,13,14,16,17,18,15,12,13,14,16,17,18,13,15,12,13,14,16,17,18],"f":[null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["level",4]],[[]],[[["level",4]],["ordering",4]],[[]],[[["box",3]],[["result",4,["box","box"]],["box",3],["box",3]]],[[["box",3]],[["result",4,["box","box"]],["box",3],["box",3]]],[[["box",3]],[["result",4,["box","box"]],["box",3],["box",3]]],[[["box",3]],[["result",4,["box","box"]],["box",3],["box",3]]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[["level",4]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["bool",15]],[[],["bool",15]],[[],["bool",15]],[[],["bool",15]],[[],["bool",15]],[[],["bool",15]],[[],["iter",3]],[[],["level",4]],[[],[["option",4,["level"]],["level",4]]],[[]],[[],["option",4]],[[],["option",4]],[[["level",4]],[["option",4,["ordering"]],["ordering",4]]],[[],[["span",3],["option",4,["span"]]]],[[]],[[],[["option",4,["box"]],["box",3,["iterator"]]]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,null,null,[[["usize",15],["diagnostics",3]],["usize",15]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["lexingerror",3]],[[],["token",3]],[[],["tokenstream",3]],[[]],[[]],[[]],[[["token",3]],["ordering",4]],[[],[["lexingerror",3],["result",4,["token","lexingerror"]],["token",3]]],[[],["lexingerror",3]],[[["token",3]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[["reader",3],["diagnostics",3]],[["token",3],["result",4,["token","lexingerror"]],["lexingerror",3]]],[[]],[[]],[[]],[[]],[[],["bool",15]],[[],["bool",15]],null,[[["token",3]],["bool",15]],[[["source",3],["diagnostics",3]]],[[["diagnostics",3]],["bool",15]],[[["token",3]],[["option",4,["ordering"]],["ordering",4]]],[[],["usize",15]],[[],["bool",15]],[[["usize",15]],["usize",15]],[[],["source",3]],null,[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,null,null,null,null,null,[[["usize",15]],["usize",15]],[[]],[[]],[[],["str",15]],[[],["str",15]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["location",3]],[[],["reader",3]],[[],["span",3]],[[],["spancontent",3]],[[],["source",3]],[[]],[[]],[[]],[[]],[[]],[[["location",3]],["ordering",4]],[[["span",3]],["ordering",4]],[[],["ordering",4]],[[],["ordering",4]],[[],["usize",15]],[[],["spancontent",3]],[[],["str",15]],[[],[["str",15],["option",4,["str"]]]],[[["usize",15]],[["str",15],["option",4,["str"]]]],[[],["str",15]],[[],["location",3]],[[["location",3]],["bool",15]],[[["span",3]],["bool",15]],[[],["bool",15]],[[],["bool",15]],[[],["bool",15]],[[],["span",3]],[[["str",15]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["source",3]],["option",4]],[[],["option",4]],[[]],[[]],[[]],[[]],[[["source",3]]],[[["source",3]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["bool",15]],[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],[[]],[[],["span",3]],[[],["location",3]],[[]],[[],["usize",15]],[[],["str",15]],[[["location",3]],["bool",15]],[[["span",3]],["bool",15]],[[]],[[],["newlineindices",3]],[[],["bool",15]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[["location",3]],[["option",4,["ordering"]],["ordering",4]]],[[["span",3]],[["option",4,["ordering"]],["ordering",4]]],[[],[["ordering",4],["option",4,["ordering"]]]],[[],[["ordering",4],["option",4,["ordering"]]]],[[],[["ordering",4],["option",4,["ordering"]]]],[[],["usize",15]],[[],["usize",15]],[[],["bool",15]],[[],["reader",3]],[[["usize",15]],["usize",15]],[[],["segmentindices",3]],[[]],[[]],[[]],[[],["source",3]],[[],["source",3]],[[],["source",3]],[[],["span",3]],[[],["span",3]],[[],["location",3]],[[]],[[]],[[]],[[]],[[]],[[],["string",3]],[[],["string",3]],[[],["string",3]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["option",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]]],"p":[[4,"Level"],[3,"Diagnostics"],[3,"IntoIter"],[3,"Iter"],[8,"Diagnostic"],[8,"Lexer"],[3,"TokenStream"],[3,"LexingError"],[3,"Token"],[8,"TokenKind"],[8,"SourceIndex"],[3,"Reader"],[3,"Span"],[3,"SpanContent"],[3,"Location"],[3,"Source"],[3,"SegmentIndices"],[3,"NewlineIndices"]]},\
"unicode_segmentation":{"doc":"Iterators which split strings on Grapheme Cluster, Word or …","t":[3,4,3,3,13,13,13,13,17,3,3,3,3,8,3,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,10,10,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,10,10,12],"n":["GraphemeCursor","GraphemeIncomplete","GraphemeIndices","Graphemes","InvalidOffset","NextChunk","PreContext","PrevChunk","UNICODE_VERSION","USentenceBoundIndices","USentenceBounds","UWordBoundIndices","UWordBounds","UnicodeSegmentation","UnicodeSentences","UnicodeWordIndices","UnicodeWords","as_str","as_str","as_str","as_str","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone","clone","clone","clone","clone","cur_cursor","eq","fmt","fmt","fmt","from","from","from","from","from","from","from","from","from","from","from","grapheme_indices","graphemes","into","into","into","into","into","into","into","into","into","into","into","into_iter","into_iter","into_iter","into_iter","into_iter","into_iter","into_iter","into_iter","into_iter","is_boundary","ne","new","next","next","next","next","next","next","next","next","next","next_back","next_back","next_back","next_back","next_back","next_back","next_boundary","prev_boundary","provide_context","set_cursor","size_hint","size_hint","size_hint","size_hint","size_hint","size_hint","split_sentence_bound_indices","split_sentence_bounds","split_word_bound_indices","split_word_bounds","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","unicode_sentences","unicode_word_indices","unicode_words","0"],"q":["unicode_segmentation","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","unicode_segmentation::GraphemeIncomplete"],"d":["Cursor-based segmenter for grapheme clusters.","An error return indicating that not enough content was …","External iterator for grapheme clusters and byte offsets.","External iterator for a string’s grapheme clusters.","An error returned when the chunk given does not contain …","When requesting <code>next_boundary</code>, the cursor is moving past …","More pre-context is needed. The caller should call …","When requesting <code>prev_boundary</code>, the cursor is moving past …","The version of Unicode that this version of …","External iterator for sentence boundaries and byte offsets.","External iterator for a string’s sentence boundaries.","External iterator for word boundaries and byte offsets.","External iterator for a string’s word boundaries.","Methods for segmenting strings according to Unicode …","An iterator over the substrings of a string which, after …","An iterator over the substrings of a string which, after …","An iterator over the substrings of a string which, after …","View the underlying data (the part yet to be iterated) as …","View the underlying data (the part yet to be iterated) as …","View the underlying data (the part yet to be iterated) as …","View the underlying data (the part yet to be iterated) as …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","The current offset of the cursor. Equal to the last value …","","","","","","","","","","","","","","","","Returns an iterator over the grapheme clusters of <code>self</code> and …","Returns an iterator over the grapheme clusters of <code>self</code>.","","","","","","","","","","","","","","","","","","","","","Determine whether the current cursor location is a …","","Create a new cursor. The string and initial offset are …","","","","","","","","","","","","","","","","Find the next boundary after the current cursor position. …","Find the previous boundary after the current cursor …","Provide additional pre-context when it is needed to decide …","Set the cursor to a new location in the same string.","","","","","","","Returns an iterator over substrings of <code>self</code>, split on …","Returns an iterator over substrings of <code>self</code> separated on …","Returns an iterator over substrings of <code>self</code>, split on …","Returns an iterator over substrings of <code>self</code> separated on …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns an iterator over substrings of <code>self</code> separated on …","Returns an iterator over the words of <code>self</code>, separated on …","Returns an iterator over the words of <code>self</code>, separated on …",""],"i":[0,0,0,0,1,1,1,1,0,0,0,0,0,0,0,0,0,2,3,4,5,6,7,2,3,8,1,4,5,9,10,11,6,7,2,3,8,1,4,5,9,10,11,2,3,8,4,5,9,10,11,8,1,3,8,1,6,7,2,3,8,1,4,5,9,10,11,12,12,6,7,2,3,8,1,4,5,9,10,11,6,7,2,3,4,5,9,10,11,8,1,8,6,7,2,3,4,5,9,10,11,6,7,2,3,4,5,8,8,8,8,2,3,4,5,10,11,12,12,12,12,6,7,2,3,8,1,4,5,9,10,11,6,7,2,3,8,1,4,5,9,10,11,6,7,2,3,8,1,4,5,9,10,11,12,12,12,13],"f":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[],["str",15]],[[],["str",15]],[[],["str",15]],[[],["str",15]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["graphemeindices",3]],[[],["graphemes",3]],[[],["graphemecursor",3]],[[],["uwordbounds",3]],[[],["uwordboundindices",3]],[[],["unicodesentences",3]],[[],["usentencebounds",3]],[[],["usentenceboundindices",3]],[[],["usize",15]],[[["graphemeincomplete",4]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["bool",15]],["graphemeindices",3]],[[["bool",15]],["graphemes",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["str",15],["usize",15]],[["result",4,["bool","graphemeincomplete"]],["graphemeincomplete",4],["bool",15]]],[[["graphemeincomplete",4]],["bool",15]],[[["usize",15],["bool",15]],["graphemecursor",3]],[[],[["str",15],["option",4,["str"]]]],[[],["option",4]],[[],["option",4]],[[],[["str",15],["option",4,["str"]]]],[[],[["str",15],["option",4,["str"]]]],[[],["option",4]],[[],[["str",15],["option",4,["str"]]]],[[],[["str",15],["option",4,["str"]]]],[[],["option",4]],[[],[["str",15],["option",4,["str"]]]],[[],["option",4]],[[],["option",4]],[[],[["str",15],["option",4,["str"]]]],[[],[["str",15],["option",4,["str"]]]],[[],["option",4]],[[["str",15],["usize",15]],[["result",4,["option","graphemeincomplete"]],["graphemeincomplete",4],["option",4,["usize"]]]],[[["str",15],["usize",15]],[["result",4,["option","graphemeincomplete"]],["graphemeincomplete",4],["option",4,["usize"]]]],[[["str",15],["usize",15]]],[[["usize",15]]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["usentenceboundindices",3]],[[],["usentencebounds",3]],[[],["uwordboundindices",3]],[[],["uwordbounds",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["unicodesentences",3]],[[],["unicodewordindices",3]],[[],["unicodewords",3]],null],"p":[[4,"GraphemeIncomplete"],[3,"GraphemeIndices"],[3,"Graphemes"],[3,"UWordBounds"],[3,"UWordBoundIndices"],[3,"UnicodeWords"],[3,"UnicodeWordIndices"],[3,"GraphemeCursor"],[3,"UnicodeSentences"],[3,"USentenceBounds"],[3,"USentenceBoundIndices"],[8,"UnicodeSegmentation"],[13,"PreContext"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};