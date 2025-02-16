import './Sidebar.css';
import FlipMove from 'react-flip-move';
import People from '../People/People'

const Sidebar = () => {
    return (
        <div className='sidebar__list'>
            {/* List of users */}
            <FlipMove>
                <People
                    key={id}
                    name={name}
                />
            </FlipMove>
        </div>   
    );
}

export default Sidebar;