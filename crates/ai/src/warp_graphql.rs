pub mod full_source_code_embedding {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EmbeddingConfig {
        OpenaiTextSmall3256,
        VoyageCode3512,
        Voyage35512,
        Voyage35Lite512,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct NodeHash(pub String);

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct ContentHash(pub String);

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct RepoMetadata {
        pub path: Option<String>,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Fragment {
        pub content: String,
        pub content_hash: ContentHash,
    }
}

pub mod queries {
    pub mod rerank_fragments {
        use crate::warp_graphql::full_source_code_embedding::ContentHash;

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct FragmentLocationInput {
            pub byte_start: i32,
            pub byte_end: i32,
            pub file_path: String,
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct RerankFragmentInput {
            pub content: String,
            pub content_hash: ContentHash,
            pub location: FragmentLocationInput,
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct RerankFragment {
            pub content: String,
            pub content_hash: ContentHash,
            pub location: FragmentLocationInput,
        }
    }
}
