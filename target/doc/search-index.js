var searchIndex = JSON.parse('{\
"block_game_clone_backend":{"doc":"Block Game Clone Backend","t":"AAAAAAAAAADENDENNNNNNLLLLLLLLLLMMLLLLLLLLLLLLLLLLLLLLLLLLLLMLLLLLLLLLLLLLLLLLLLMLLLLLLLLLLLLLLLLLLLLLMLMLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLFDMLLLLLLLLLLLLLLLLLLLLLLLLMLLLLLLLLNENNENNNENNLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLDLLMLLLLLLLLLLLLLMMLLLLLLLLLMLLLMLLLLLLLLLLNNNNNNDENNNNNLLLLLLLLLMLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLNNELLLLLLLLLLLLLLLLLLLLLLLLLLLDLLLLLLLLLLLLLLLLLLLMLLMLLLLLLLLLLDLLLLLMLLLLLLLLLLLLLLLLMLLLLLLLLLLLLENNNNNNNENNDNNLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLFFLLLLLLLLLLLLLLLLLLLLLFFFF","n":["board","board_state","game_mode","game_state","piece","player","player_hand_state","player_state","ts_interop","utils","Board","BoardCell","Collision","Cube","CubeError","Empty","NotTouchingPiece","OutOfBounds","OutOfBounds","Player","Unsupported","add_cubes","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","calculate_score","cells","center","check_in_bounds_no_collision","check_touches_piece","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","decl","decl","decl","decl","default","default","dependencies","dependencies","dependencies","dependencies","deserialize","deserialize","deserialize","deserialize","eq","eq","error","fmt","fmt","fmt","fmt","from","from","from","from","from_subset","from_subset","from_subset","from_subset","get","get_adjacent_cells","get_available_positions","get_from_index","get_from_index_mut","get_highest_player","get_mut","height_limits","inline","inline","inline","inline","inline_flattened","inline_flattened","into","into","into","into","is_in_subset","is_in_subset","is_in_subset","is_in_subset","map_to_heights","name","name","name","name","new","new_board_from_2d_heights","player","player_has_played","position","serialize","serialize","serialize","serialize","supports","to_owned","to_owned","to_owned","to_owned","to_subset","to_subset","to_subset","to_subset","to_subset_unchecked","to_subset_unchecked","to_subset_unchecked","to_subset_unchecked","transparent","transparent","transparent","transparent","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","v3_to_index","BoardState","board","borrow","borrow_mut","calculate_score","check_cube_supported","check_in_bounds_no_collision","check_piece_placement","clear_previewed_piece","clone","clone_into","decl","default","dependencies","deserialize","fmt","from","from_subset","inline","inline_flattened","into","is_in_subset","name","new","play_selected_piece","preview_piece","previewed_piece","serialize","to_owned","to_subset","to_subset_unchecked","transparent","try_from","try_into","type_id","FourByFiveByTwo","GameMode","Pyramid","Solitaire","SolitaireMap","Stairs","Tower","TwoPlayer","TwoPlayerMap","VSGreedyAI","Wall","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone_into","clone_into","clone_into","decl","decl","decl","default","dependencies","dependencies","dependencies","deserialize","deserialize","deserialize","eq","eq","eq","equivalent","equivalent","equivalent","fmt","fmt","fmt","from","from","from","from_subset","from_subset","from_subset","inline","inline","inline","into","into","into","is_in_subset","is_in_subset","is_in_subset","name","name","name","serialize","serialize","serialize","to_owned","to_owned","to_owned","to_subset","to_subset","to_subset","to_subset_unchecked","to_subset_unchecked","to_subset_unchecked","transparent","transparent","transparent","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","GameState","apply_action","available_move_exists","board_state","borrow","borrow_mut","clear_selected_piece","clone","clone_into","decl","default","dependencies","deserialize","determine_game_ended","fmt","from","from_subset","game_ended","game_mode","inline","inline_flattened","into","is_in_subset","make_greedy_ai_move","name","new","pass_turn","play_previewed_piece","player_state","preview_piece","reset","rotate_selected_piece","score","select_piece","serialize","set_selected_piece_origin","to_owned","to_subset","to_subset_unchecked","transparent","try_from","try_into","type_id","Corner","L","LeftScrew","OneByFour","OneByThree","OneByTwo","Piece","PieceName","RightScrew","ShortL","T","TwoByTwo","Z","apply_rotation","borrow","borrow","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","coords","decl","decl","dependencies","dependencies","deserialize","deserialize","eq","equivalent","fmt","fmt","from","from","from_subset","from_subset","from_vec_i8_array","get_available_piece_rotations","get_moved_copy","hash","inline","inline","inline_flattened","into","into","is_in_subset","is_in_subset","name","name","rotate","serialize","serialize","set_origin","supports","to_owned","to_owned","to_subset","to_subset","to_subset_unchecked","to_subset_unchecked","transparent","transparent","try_from","try_from","try_into","try_into","type_id","type_id","P1","P2","Player","borrow","borrow_mut","clone","clone_into","decl","default","dependencies","deserialize","eq","equivalent","fmt","from","from_subset","get_other","hash","inline","into","is_in_subset","name","serialize","to_owned","to_subset","to_subset_unchecked","transparent","try_from","try_into","type_id","PlayerHandState","borrow","borrow_mut","clear_selected_piece","clone","clone_into","decl","default","dependencies","deserialize","fmt","from","from_subset","get_available_piece_rotations","get_selected_piece","inline","inline_flattened","into","is_in_subset","name","pieces","play_selected_piece","rotate_selected_piece","selected_piece","serialize","set_selected_piece","set_selected_piece_origin","to_owned","to_subset","to_subset_unchecked","transparent","try_from","try_into","type_id","PlayerState","borrow","borrow_mut","clear_selected_piece","clone","clone_into","current_player","decl","default","dependencies","deserialize","fmt","from","from_subset","get_available_piece_rotations","get_selected_piece","inline","inline_flattened","into","is_in_subset","name","new","play_selected_piece","players","rotate_selected_piece","select_piece","serialize","set_selected_piece_origin","to_owned","to_subset","to_subset_unchecked","toggle_current_player","transparent","try_from","try_into","type_id","Action","ClearSelectedPiece","MakeGreedyAIMove","PassTurn","PlayPreviewedPiece","PreviewPiece","Reset","RotateSelectedPiece","RotationAxis","SelectPiece","SetSelectedPieceOrigin","V3","X","Y","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","decl","decl","decl","dependencies","dependencies","dependencies","deserialize","deserialize","deserialize","from","from","from","from_subset","from_subset","from_subset","inline","inline","inline","into","into","into","is_in_subset","is_in_subset","is_in_subset","name","name","name","new_game","next_game_state","serialize","serialize","serialize","to_subset","to_subset","to_subset","to_subset_unchecked","to_subset_unchecked","to_subset_unchecked","transparent","transparent","transparent","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","log","log_many","log_u32","set_panic_hook"],"q":[[0,"block_game_clone_backend"],[10,"block_game_clone_backend::board"],[138,"block_game_clone_backend::board_state"],[173,"block_game_clone_backend::game_mode"],[257,"block_game_clone_backend::game_state"],[300,"block_game_clone_backend::piece"],[369,"block_game_clone_backend::player"],[399,"block_game_clone_backend::player_hand_state"],[433,"block_game_clone_backend::player_state"],[469,"block_game_clone_backend::ts_interop"],[539,"block_game_clone_backend::utils"]],"d":["Contains Board","Contains BoardState","Contains GameMode","Contains GameState which represents the state of the game","Contains Piece, PieceName","Contains Player enum","contains PlayerHandState","manages information about players","Functions and types for Rust (as WASM) &lt;-&gt; TS interop","","Represents the state of the board","","","","","","","","","","","","","","","","","","","","","","useful for centering a camera","","implements game rule requiring players to play touching …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","returns all available positions on the board. Used for …","","","Returns the highest player for a given column, used for …","","used to construct, show available space to player","","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","determines whether or not a player has played","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","The state of the board, including move preview","the current state of the board, available space, pieces …","","","Returns the current score","Checks that a given cube of the potential play is …","performs in bounds and collision checks","returns Vec of cubes with position and possible error …","Clears the previewed piece","","","","","","","","Returns the argument unchanged.","","","","Calls <code>U::from(self)</code>.","","","","Plays the selected piece if the previewed move is valid","Previews a piece placement","used to show a player the result of a possible move, move …","","","","","","","","","","Represents game mode and map","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Represents the state of the game","Takes an Action performed by a player and updates the …","","","","","","","","","","","","","","Returns the argument unchanged.","","","","","","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","","","Represents a piece as a Vec of offsets as …","identifies pieces","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","","","","Give all possible rotations of the current piece","applies vector translation to constituent cubes","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","Rotates the piece about the input RotationAxis","","","Imagine picking up a Polycube by one of the cubes","Does this piece support a given position. All cubes must …","","","","","","","","","","","","","","","","","enum representing a player","","","","","","","","","","","","Returns the argument unchanged.","","toggles between values","","","Calls <code>U::from(self)</code>.","","","","","","","","","","","represents the pieces in a players hand, their …","","","","","","","","","","","Returns the argument unchanged.","","returns all possible rotations of all available pieces in …","returns the selected piece if a piece is selected","","","Calls <code>U::from(self)</code>.","","","The pieces in a player’s hand. Pieces already played are …","marks the selected piece as unavailable","Rotates the selected piece PI / 2 about the rotation axis …","The piece currently selected by the player, if one is …","","sets a piece as selected by the player","sets which cube among the cubes comprising the selected …","","","","","","","","manages information about players","","","","","","","","","","","","Returns the argument unchanged.","","","","","","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","Action enum defines all actions that could be performed by …","","","","","","","","Which axis the current player has chosen to rotate the …","","","A newtype wrapper around a <code>nalgebra::Vector3&lt;f32&gt;</code>","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","used for generating a new game from WASM","Given a GameState and Action as &amp;str’s in WASM, returns …","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,0,0,0,0,0,0,0,0,9,0,0,12,9,12,9,12,9,1,1,12,9,2,1,12,9,2,1,1,1,1,1,1,12,9,2,1,12,9,2,1,12,9,2,1,12,1,12,9,2,1,12,9,2,12,9,2,1,12,9,2,1,12,9,2,1,12,9,2,1,1,1,1,1,1,1,1,1,12,9,2,1,2,1,12,9,2,1,12,9,2,1,1,12,9,2,1,1,2,1,2,1,12,9,2,1,1,12,9,2,1,12,9,2,1,12,9,2,1,12,9,2,1,12,9,2,1,12,9,2,1,12,9,2,0,0,24,24,24,24,24,24,24,24,24,24,24,24,24,24,24,24,24,24,24,24,24,24,24,24,24,24,24,24,24,24,24,24,24,24,26,0,19,21,0,19,19,21,0,21,19,21,26,19,21,26,19,21,26,19,21,26,19,21,26,19,21,21,26,19,21,26,19,21,26,19,21,26,19,21,26,19,21,26,19,21,26,19,21,26,19,21,26,19,21,26,19,21,26,19,21,26,19,21,26,19,21,26,19,21,26,19,21,26,19,21,26,19,21,26,19,21,26,19,0,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,30,30,30,30,30,30,0,0,30,30,30,30,30,25,25,30,25,30,25,30,25,30,25,25,30,25,30,25,30,30,30,25,30,25,30,25,30,25,25,25,30,25,30,25,25,30,25,30,25,30,25,25,30,25,25,25,30,25,30,25,30,25,30,25,30,25,30,25,30,4,4,0,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,0,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,34,0,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,35,0,28,28,28,28,28,28,28,0,28,28,0,29,29,36,28,29,36,28,29,36,28,29,36,28,29,36,28,29,36,28,29,36,28,29,36,28,29,36,28,29,36,28,29,36,28,29,0,0,36,28,29,36,28,29,36,28,29,36,28,29,36,28,29,36,28,29,36,28,29,0,0,0,0],"f":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[1,[3,[2]]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[1,[[6,[4,5]]]],0,0,[[1,[8,[7]]],[[10,[9]]]],[[1,[3,[2]]],[[11,[[3,[2]],[3,[2]]]]]],[1,1],[12,12],[9,9],[2,2],[[]],[[]],[[]],[[]],[[],13],[[],13],[[],13],[[],13],[[],1],[[],12],[[],[[3,[14]]]],[[],[[3,[14]]]],[[],[[3,[14]]]],[[],[[3,[14]]]],[15,[[11,[1]]]],[15,[[11,[12]]]],[15,[[11,[9]]]],[15,[[11,[2]]]],[[12,12],16],[[9,9],16],0,[[1,17],18],[[12,17],18],[[9,17],18],[[2,17],18],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[1,[8,[7]]],[[10,[12]]]],[[1,[8,[7]]],[[3,[12]]]],[1,[[3,[[8,[7]]]]]],[1,[[10,[12]]]],[1,[[10,[12]]]],[[1,5,5,5],[[10,[4]]]],[[1,[8,[7]]],[[10,[12]]]],0,[[],13],[[],13],[[],13],[[],13],[[],13],[[],13],[[]],[[]],[[]],[[]],[[],16],[[],16],[[],16],[[],16],[19,[[3,[[3,[20]]]]]],[[],13],[[],13],[[],13],[[],13],[21,1],[[[3,[[3,[20]]]]],1],0,[[1,4],[[10,[4]]]],0,[[1,22],11],[[12,22],11],[[9,22],11],[[2,22],11],[[1,[8,[7]]],16],[[]],[[]],[[]],[[]],[[],10],[[],10],[[],10],[[],10],[[]],[[]],[[]],[[]],[[],16],[[],16],[[],16],[[],16],[[],11],[[],11],[[],11],[[],11],[[],11],[[],11],[[],11],[[],11],[[],23],[[],23],[[],23],[[],23],[[[8,[7]]],10],0,0,[[]],[[]],[24,[[6,[4,5]]]],[[24,2,25],2],[[24,2],2],[[24,4,25,[8,[7]]],[[3,[2]]]],[24],[24,24],[[]],[[],13],[[],24],[[],[[3,[14]]]],[15,[[11,[24]]]],[[24,17],18],[[]],[[]],[[],13],[[],13],[[]],[[],16],[[],13],[21,24],[24,11],[[24,4,25,[8,[7]]]],0,[[24,22],11],[[]],[[],10],[[]],[[],16],[[],11],[[],11],[[],23],0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[21,21],[26,26],[19,19],[[]],[[]],[[]],[[],13],[[],13],[[],13],[[],21],[[],[[3,[14]]]],[[],[[3,[14]]]],[[],[[3,[14]]]],[15,[[11,[21]]]],[15,[[11,[26]]]],[15,[[11,[19]]]],[[21,21],16],[[26,26],16],[[19,19],16],[[],16],[[],16],[[],16],[[21,17],18],[[26,17],18],[[19,17],18],[[]],[[]],[[]],[[]],[[]],[[]],[[],13],[[],13],[[],13],[[]],[[]],[[]],[[],16],[[],16],[[],16],[[],13],[[],13],[[],13],[[21,22],11],[[26,22],11],[[19,22],11],[[]],[[]],[[]],[[],10],[[],10],[[],10],[[]],[[]],[[]],[[],16],[[],16],[[],16],[[],11],[[],11],[[],11],[[],11],[[],11],[[],11],[[],23],[[],23],[[],23],0,[[27,28]],[[27,4],16],0,[[]],[[]],[27],[27,27],[[]],[[],13],[[],27],[[],[[3,[14]]]],[15,[[11,[27]]]],[27],[[27,17],18],[[]],[[]],0,0,[[],13],[[],13],[[]],[[],16],[27],[[],13],[21,27],[27],[27,11],0,[[27,[8,[7]]]],[27],[[27,29]],0,[[27,30]],[[27,22],11],[[27,[8,[7]]]],[[]],[[],10],[[]],[[],16],[[],11],[[],11],[[],23],0,0,0,0,0,0,0,0,0,0,0,0,0,[[25,[31,[7]]],25],[[]],[[]],[[]],[[]],[25,25],[30,30],[[]],[[]],0,[[],13],[[],13],[[],[[3,[14]]]],[[],[[3,[14]]]],[15,[[11,[25]]]],[15,[[11,[30]]]],[[30,30],16],[[],16],[[25,17],18],[[30,17],18],[[]],[[]],[[]],[[]],[[[3,[[32,[5]]]]],25],[25,[[3,[25]]]],[[25,[8,[7]]],25],[[30,33]],[[],13],[[],13],[[],13],[[]],[[]],[[],16],[[],16],[[],13],[[],13],[[25,29]],[[25,22],11],[[30,22],11],[[25,[8,[7]]]],[[25,[8,[7]]],16],[[]],[[]],[[],10],[[],10],[[]],[[]],[[],16],[[],16],[[],11],[[],11],[[],11],[[],11],[[],23],[[],23],0,0,0,[[]],[[]],[4,4],[[]],[[],13],[[],4],[[],[[3,[14]]]],[15,[[11,[4]]]],[[4,4],16],[[],16],[[4,17],18],[[]],[[]],[4,4],[[4,33]],[[],13],[[]],[[],16],[[],13],[[4,22],11],[[]],[[],10],[[]],[[],16],[[],11],[[],11],[[],23],0,[[]],[[]],[34],[34,34],[[]],[[],13],[[],34],[[],[[3,[14]]]],[15,[[11,[34]]]],[[34,17],18],[[]],[[]],[34,3],[34,[[10,[25]]]],[[],13],[[],13],[[]],[[],16],[[],13],0,[34],[[34,29]],0,[[34,22],11],[[34,30]],[[34,[8,[7]]]],[[]],[[],10],[[]],[[],16],[[],11],[[],11],[[],23],0,[[]],[[]],[35],[35,35],[[]],0,[[],13],[[],35],[[],[[3,[14]]]],[15,[[11,[35]]]],[[35,17],18],[[]],[[]],[35,3],[35,10],[[],13],[[],13],[[]],[[],16],[[],13],[21,35],[35],0,[[35,29]],[[35,30]],[[35,22],11],[[35,[8,[7]]]],[[]],[[],10],[[]],[35],[[],16],[[],11],[[],11],[[],23],0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[],13],[[],13],[[],13],[[],[[3,[14]]]],[[],[[3,[14]]]],[[],[[3,[14]]]],[15,[[11,[36]]]],[15,[[11,[28]]]],[15,[[11,[29]]]],[[]],[[]],[[]],[[]],[[]],[[]],[[],13],[[],13],[[],13],[[]],[[]],[[]],[[],16],[[],16],[[],16],[[],13],[[],13],[[],13],[37,13],[[37,37],13],[[36,22],11],[[28,22],11],[[29,22],11],[[],10],[[],10],[[],10],[[]],[[]],[[]],[[],16],[[],16],[[],16],[[],11],[[],11],[[],11],[[],11],[[],11],[[],11],[[],23],[[],23],[[],23],[37],[[37,37]],[38],[[]]],"c":[],"p":[[3,"Board"],[3,"Cube"],[3,"Vec"],[4,"Player"],[15,"i8"],[3,"HashMap"],[15,"f32"],[6,"Vector3"],[4,"CubeError"],[4,"Option"],[4,"Result"],[4,"BoardCell"],[3,"String"],[3,"Dependency"],[8,"Deserializer"],[15,"bool"],[3,"Formatter"],[6,"Result"],[4,"TwoPlayerMap"],[15,"usize"],[4,"GameMode"],[8,"Serializer"],[3,"TypeId"],[3,"BoardState"],[3,"Piece"],[4,"SolitaireMap"],[3,"GameState"],[4,"Action"],[4,"RotationAxis"],[4,"PieceName"],[6,"Rotation3"],[15,"array"],[8,"Hasher"],[3,"PlayerHandState"],[3,"PlayerState"],[3,"V3"],[15,"str"],[15,"u32"]]},\
"build_automation":{"doc":"","t":"FFFFFF","n":["add_types_to_package_json","build_pkg","document","generate_index_file_for_ts_types","generate_ts_types","main"],"q":[[0,"build_automation"]],"d":["","","","","",""],"i":[0,0,0,0,0,0],"f":[[[]],[[]],[[]],[[]],[[]],[[]]],"c":[],"p":[]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
