import './Group.css';

const Group = () => {
	return (
        <div classname="group-container">
            <h3>Create a new group</h3>
            <div>
                <form>
                <div>
                <input type="text" placeholder="group name"></input>
                </div>
                <div>
                    <button type="submit">Create Group</button>
                </div>
                </form>
            </div>
        </div>
	);
};

export default Group;