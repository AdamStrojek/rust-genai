use crate::{ChatMessage, ChatResponse, ChatRole, StreamItem};
use async_openai::types as oa_types;

// region:    --- From [genai] to [raw]

/// From ChatMsg to async_openai ChatCompletionRequestMessage
impl From<ChatMessage> for oa_types::ChatCompletionRequestMessage {
	fn from(chat_msg: ChatMessage) -> Self {
		let role = chat_msg.role.into();
		let content = chat_msg.content;

		match chat_msg.extra {
			None => oa_types::ChatCompletionRequestUserMessage {
				content: content.into(),
				role,
				name: None,
			}
			.into(),
			_ => todo!("chat_msg other not supported yet"),
		}
	}
}

/// From ChatRole to async_openai Role
impl From<ChatRole> for oa_types::Role {
	fn from(chat_role: ChatRole) -> Self {
		match chat_role {
			ChatRole::System => oa_types::Role::System,
			ChatRole::User => oa_types::Role::User,
			ChatRole::Assistant => oa_types::Role::Assistant,
			// TODO: need to decide what to do with function
			ChatRole::Tool => oa_types::Role::Tool,
		}
	}
}

// endregion: --- From [genai] to [raw]

// region:    --- From [raw] to [genai]

impl From<oa_types::CreateChatCompletionResponse> for ChatResponse {
	fn from(raw_res: oa_types::CreateChatCompletionResponse) -> Self {
		// NOTE: For now, only simport single completion choice (as specified in the `into_oa_chat_req` )
		let choice = raw_res.choices.into_iter().next();
		let response = choice.and_then(|choice| choice.message.content);

		ChatResponse { content: response }
	}
}

impl From<oa_types::CreateChatCompletionStreamResponse> for StreamItem {
	fn from(value: oa_types::CreateChatCompletionStreamResponse) -> Self {
		let first_choice = value.choices.into_iter().next();
		let response = first_choice.map(|c| c.delta).and_then(|d| d.content);
		StreamItem { content: response }
	}
}

// endregion: --- From [raw] to [genai]