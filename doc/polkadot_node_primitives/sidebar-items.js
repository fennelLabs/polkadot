window.SIDEBAR_ITEMS = {"constant":[["ACTIVE_DURATION_SECS","The choice here is fairly arbitrary. But any dispute that concluded more than a few minutes ago is not worth considering anymore. Changing this value has little to no bearing on consensus, and really only affects the work that the node might do on startup during periods of many disputes."],["APPROVAL_EXECUTION_TIMEOUT","The amount of time to spend on execution during approval or disputes."],["BACKING_EXECUTION_TIMEOUT","The amount of time to spend on execution during backing."],["DISPUTE_CANDIDATE_LIFETIME_AFTER_FINALIZATION","How many blocks after finalization an information about backed/included candidate should be kept."],["DISPUTE_WINDOW","It would be nice to draw this from the chain state, but we have no tools for it right now. On Polkadot this is 1 day, and on Kusama it’s 6 hours."],["MAX_FINALITY_LAG","Linked to `MAX_FINALITY_LAG` in relay chain selection, `MAX_HEADS_LOOK_BACK` in `approval-voting` and `MAX_BATCH_SCRAPE_ANCESTORS` in `dispute-coordinator`"],["POV_BOMB_LIMIT","The bomb limit for decompressing PoV blobs."],["VALIDATION_CODE_BOMB_LIMIT","The bomb limit for decompressing code blobs."]],"enum":[["BabeAllowedSlots","Types of allowed slots."],["DisputeMessageCheckError","Things that can go wrong when constructing a `DisputeMessage`."],["DisputeStatus","The status of dispute."],["InvalidCandidate","Candidate invalidity details"],["MaybeCompressedPoV","A type that represents a maybe compressed [`PoV`]."],["MerkleProofError","Possible errors when converting from `Vec<Vec<u8>>` into [`Proof`]."],["Statement","A statement, where the candidate receipt is included in the `Seconded` variant."],["ValidationResult","Result of the validation of the candidate."]],"fn":[["dispute_is_inactive","Returns true if the dispute has concluded for longer than [`ACTIVE_DURATION_SECS`]."],["maybe_compress_pov","Compress a PoV, unless it exceeds the [`POV_BOMB_LIMIT`]."]],"macro":[["new_session_window_size","Create a new checked `SessionWindowSize` which cannot be 0."]],"mod":[["approval","Types relevant for approval."],["disputes","Disputes related types."]],"struct":[["AvailableData","This is the data we keep available for each candidate included in the relay chain."],["BabeEpoch","BABE epoch information"],["BabeEpochConfiguration","Configuration data used by the BABE consensus engine that may change with epochs."],["BlockData","Parachain block data."],["Collation","The output of a collator."],["CollationGenerationConfig","Configuration for the collation generator"],["CollationResult","Result of the [`CollatorFn`] invocation."],["CollationSecondedSignal","Signal that is being returned when a collation was seconded by a validator."],["DisputeMessage","A dispute initiating/participating message that have been built from signed statements."],["ErasureChunk","A chunk of erasure-encoded block data."],["PoV","A Proof-of-Validity"],["Proof","This is a convenience type to allow the Erasure chunk proof to Decode into a nested BoundedVec"],["SessionWindowSize","Type of a session window size."],["UncheckedDisputeMessage","A `DisputeMessage` where signatures of statements have not yet been checked."]],"type":[["BlockWeight","The cumulative weight of a block in a fork-choice rule."],["CollatorFn","Collation function."],["HorizontalMessages",""],["SignedFullStatement","A statement, the corresponding signature, and the index of the sender."],["Timestamp","Timestamp based on the 1 Jan 1970 UNIX base, which is persistent across node restarts and OS reboots."],["UncheckedSignedFullStatement","Variant of `SignedFullStatement` where the signature has not yet been verified."],["UpwardMessages",""]]};