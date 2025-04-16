import { useEffect, useState } from 'react';
import './Chat.css';

const Chat = ({lastJsonMessage, handleSendMessage, otherUser, canvas}) => {
	const [sendCanvas, setSendCanvas] = useState()
	useEffect(()=>{
		setSendCanvas(canvas?.sent)
	},[canvas?.sent])
	return (
		<div className="main-container">
		<div className="left-container">
			<div className="chat-header">
				<p>{localStorage.getItem("userName")} (You)</p>
			</div>
			<div className="chat-body">
				<textarea
					onKeyDownCapture={e => {handleSendMessage(e); }}
					onChange={e => setSendCanvas(e.target.value)}
					value={sendCanvas}>
				</textarea>
			</div>
		</div>
		<div className="right-container">
			<div className="chat-header">
				<p>{otherUser?.name||"Other User"}</p>
			</div>
			<div className="chat-body">
				<textarea value={lastJsonMessage?.message||canvas?.received}></textarea>
			</div>
		</div>
		</div>
	);
};

export default Chat;
