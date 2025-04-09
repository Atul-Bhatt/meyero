import './Chat.css';

const Chat = ({lastJsonMessage, handleSendMessage}) => {
	return (
		<div className="main-container">
		<div className="left-container">
			<div className="chat-body">
				<textarea
					onKeyDownCapture={handleSendMessage}>
				</textarea>
			</div>
		</div>
		<div className="right-container">
			<div className="chat-body">
				<textarea value={lastJsonMessage?.data?.message}></textarea>
			</div>
		</div>
		</div>
	);
};

export default Chat;
