use hex_literal::hex;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::UncheckedInto};
use node_template_runtime::{AccountId};

pub fn endowed_accounts() -> Vec<AccountId> {
    vec![
		// 5FeyRQmjtdHoPH56ASFW76AJEP1yaQC1K9aEMvJTF9nzt9S9
		// hex!["a2bf32e50edd79c181888da41c80c67c191e9e6b29d3f2efb102ca0e2b53c558"].into(),
		hex!["3233f745d0860ed64ae9c7f4ea5c0773316fc9265199f312d3f6e8ce08255c10"].into(),
		hex!["445f574d57f768ea7e1a2f551bef4298ace99d8895d316352cfc02aececcf26c"].into(),
		hex!["c2312f7f9a8190bf76db9dc40e5ef351c4c23e3ae6540932bf2c2d485289c37b"].into(),
		hex!["688d6fa54d9ace0fa07492f3d8dfef78594130719e61c213d700c62421177c38"].into(),
	]
}

pub fn initial_authorities() -> Vec<(
    AuraId,
    GrandpaId
)> {
    vec![
		(
			// Sankar//aura
			hex!["4e743e3efea9390a89894383c8567907db13edb3fb2c1e0f8c428a745e05be77"].unchecked_into(),
			// Sankar//grandpa
			hex!["161a16204ef752deeb0002578456e77df6caa89ec463a1bf471533e93369e2b8"].unchecked_into(),
		),
		(
			// Arun//aura
			hex!["b21c79d2588ddeaf6005a28222110c62f7dd31b89c03efd46ba6b555b327eb08"].unchecked_into(),
			// Arun//grandpa
			hex!["14aa4f9c765360d5d408f1fd6563612bd30a9f50d1b3f37ac096d0a4671bcad7"].unchecked_into(),
		),
		(
			// Rakhi//aura
			hex!["8c6fb1e4c8f14e860eecdffefb9b2e132e64462e71dc877da7a906af615c0e71"].unchecked_into(),
			// Rakhi//grandpa
			hex!["b34908f44b24b3052d2324a06d12a564350083b03441bc204c8acd8642b7c844"].unchecked_into(),
		),
		(
			// Priya//aura
			hex!["069a3928909ec6de4f1f5e8cfaa42854e6e0eca6962fa54be59a284f7e7b385d"].unchecked_into(),
			// Priya//grandpa
			hex!["687e40323706755ab58d6a1b31aabedde5fbde9f682277df587af1b4e8616847"].unchecked_into(),
		)
	]
}

	