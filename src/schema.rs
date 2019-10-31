table! {
    canonical_forms (id) {
        id -> Unsigned<Integer>,
        name -> Nullable<Varchar>,
        first_letter -> Char,
        length -> Integer,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    data_sources (id) {
        id -> Integer,
        title -> Nullable<Varchar>,
        description -> Nullable<Text>,
        // logo_url -> Nullable<Varchar>,
        // web_site_url -> Nullable<Varchar>,
        // data_url -> Nullable<Varchar>,
        // refresh_period_days -> Nullable<Integer>,
        // name_strings_count -> Nullable<Integer>,
        // data_hash -> Nullable<Varchar>,
        // unique_names_count -> Nullable<Integer>,
        // created_at -> Nullable<Datetime>,
        // updated_at -> Nullable<Datetime>,
    }
}

table! {
    data_source_contributors (id) {
        id -> Integer,
        data_source_id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        data_source_admin -> Nullable<Bool>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    dwca_importers (id) {
        id -> Integer,
        data_source_id -> Nullable<Integer>,
        url -> Nullable<Varchar>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    job_logs (id) {
        id -> Integer,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        job_id -> Nullable<Integer>,
        message -> Nullable<Varchar>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    lexical_groups (id) {
        id -> Integer,
        supercedure_id -> Nullable<Integer>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    lexical_group_name_strings (name_string_id, lexical_group_id) {
        name_string_id -> Unsigned<Integer>,
        lexical_group_id -> Unsigned<Integer>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    lexical_matches (canonical_form_id, matched_canonical_form_id) {
        canonical_form_id -> Unsigned<Integer>,
        matched_canonical_form_id -> Unsigned<Integer>,
        edit_distance -> Integer,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    lexical_match_candidates (canonical_form_id, candidate_name) {
        canonical_form_id -> Unsigned<Integer>,
        candidate_name -> Varchar,
        processed -> Nullable<Bool>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    name_resolvers (id) {
        id -> Integer,
        options -> Nullable<Varchar>,
        progress_status_id -> Nullable<Integer>,
        progress_message -> Nullable<Varchar>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
        token -> Nullable<Varchar>,
    }
}

table! {
    name_strings (id) {
        id -> Unsigned<Integer>,
        name -> Nullable<Varchar>,
        normalized -> Nullable<Varchar>,
        canonical_form_id -> Nullable<Integer>,
        has_words -> Nullable<Bool>,
        uuid -> Decimal,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
        parser_version -> Nullable<Integer>,
        has_groups -> Nullable<Bool>,
        surrogate -> Nullable<Bool>,
    }
}

table! {
    name_string_indices (data_source_id, name_string_id, taxon_id) {
        data_source_id -> Integer,
        name_string_id -> Unsigned<Integer>,
        taxon_id -> Varchar,
        global_id -> Nullable<Varchar>,
        url -> Nullable<Varchar>,
        rank -> Nullable<Varchar>,
        accepted_taxon_id -> Nullable<Varchar>,
        //synonym -> Nullable<Set>,
        classification_path -> Nullable<Text>,
        classification_path_ids -> Nullable<Text>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
        nomenclatural_code_id -> Nullable<Integer>,
        local_id -> Nullable<Varchar>,
        classification_path_ranks -> Nullable<Text>,
    }
}

table! {
    name_words (id) {
        id -> Unsigned<Integer>,
        word -> Nullable<Varchar>,
        first_letter -> Char,
        length -> Nullable<Integer>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    name_word_semantic_meanings (id) {
        id -> Unsigned<Integer>,
        name_word_id -> Unsigned<Integer>,
        name_string_id -> Unsigned<Integer>,
        semantic_meaning_id -> Nullable<Integer>,
        position -> Nullable<Integer>,
        length -> Nullable<Integer>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    nomenclatural_codes (id) {
        id -> Integer,
        code -> Nullable<Varchar>,
        title -> Nullable<Varchar>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    parsed_name_strings (id) {
        id -> Unsigned<Integer>,
        canonical_form -> Nullable<Varchar>,
        parser_version -> Nullable<Varchar>,
        parsed -> Nullable<Bool>,
        pass_num -> Nullable<Integer>,
        data -> Nullable<Text>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    progress_statuses (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    reconcilers (id) {
        id -> Integer,
        batch_size -> Nullable<Integer>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    reconciler_batches (id) {
        id -> Integer,
        reconciler_id -> Nullable<Integer>,
        offset -> Nullable<Integer>,
        status -> Nullable<Integer>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    reconciler_data_sources (id) {
        id -> Integer,
        reconciler_id -> Nullable<Integer>,
        data_source_id -> Nullable<Integer>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    semantic_meanings (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    users (id) {
        id -> Integer,
        email -> Varchar,
        encrypted_password -> Varchar,
        reset_password_token -> Nullable<Varchar>,
        reset_password_sent_at -> Nullable<Datetime>,
        remember_created_at -> Nullable<Datetime>,
        sign_in_count -> Nullable<Integer>,
        current_sign_in_at -> Nullable<Datetime>,
        last_sign_in_at -> Nullable<Datetime>,
        current_sign_in_ip -> Nullable<Varchar>,
        last_sign_in_ip -> Nullable<Varchar>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    vernacular_strings (id) {
        id -> Unsigned<Integer>,
        name -> Nullable<Varchar>,
        uuid -> Decimal,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    vernacular_string_indices (data_source_id, vernacular_string_id, taxon_id) {
        data_source_id -> Integer,
        vernacular_string_id -> Unsigned<Integer>,
        taxon_id -> Varchar,
        language -> Nullable<Varchar>,
        locality -> Nullable<Varchar>,
        country_code -> Nullable<Varchar>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

allow_tables_to_appear_in_same_query!(
  canonical_forms,
  data_sources,
  data_source_contributors,
  dwca_importers,
  job_logs,
  lexical_groups,
  lexical_group_name_strings,
  lexical_matches,
  lexical_match_candidates,
  name_resolvers,
  name_strings,
  name_string_indices,
  name_words,
  name_word_semantic_meanings,
  nomenclatural_codes,
  parsed_name_strings,
  progress_statuses,
  reconcilers,
  reconciler_batches,
  reconciler_data_sources,
  semantic_meanings,
  users,
  vernacular_strings,
  vernacular_string_indices,
);
