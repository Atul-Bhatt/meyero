import './Chat.css';

const Chat = ({lastJsonMessage, handleSendMessage, currentUser}) => {
	return (
		<div className="main-container">
		<div className="left-container">
			<div className="chat-header">
				<p>You</p>
			</div>
			<div className="chat-body">
				<textarea
					onKeyDownCapture={handleSendMessage}>
				</textarea>
			</div>
		</div>
		<div className="right-container">
			<div className="chat-header">
				<p>{currentUser.name||"Other User"}</p>
			</div>
			<div className="chat-body">
				<textarea value={lastJsonMessage?.data?.message}></textarea>
			</div>
		</div>
		</div>
	);
};

export default Chat;
