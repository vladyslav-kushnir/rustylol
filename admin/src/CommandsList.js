import {useEffect, useState} from 'react';
import Command from './Command';
import Api from './api/Api';

export default function CommandsList() {
    const [commands, setCommands] = useState([]);

    let addCommand = () => {
        setCommands([...commands, { name: '', variations: { 0: { url_pattern: '' }}, isNew: true }])
    };

    let deleteCommand = (idx) => {
        let newCommands = [...commands];
        newCommands.splice(idx, 1);

        setCommands(newCommands);
    }

    useEffect(() => {
        Api.getCommands().then(commands => {
            setCommands(commands);
        });
    }, []);

    return (
        <div>
            {commands.map((x, idx) => 
                <Command key={idx} {...x} onDelete={deleteCommand.bind(this, idx)} />
            )}
            <div><button onClick={addCommand}>Add command</button></div>
        </div>
    );
}