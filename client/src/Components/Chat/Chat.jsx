import './Chat.css';

const Chat = ({lastJsonMessage, handleSendMessage}) => {
	return (
		<div className="main-container">
		<div className="left-container">
			<div className="chat-header">
				<p>Client</p>
			</div>
			<div className="chat-body">
				<input 
					onKeyDownCapture={handleSendMessage}>
				</input>
			</div>
		</div>
		<div className="right-container">
			<div className="chat-header">
				<p>Server</p>
			</div>
			<div className="chat-body">
				<input value={lastJsonMessage?.data?.message}></input>
			</div>
		</div>
		</div>
	);
};

export default Chat;
