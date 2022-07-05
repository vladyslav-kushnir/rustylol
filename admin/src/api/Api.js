class Api {
    async getCommands() {
        const res = await fetch(process.env.REACT_APP_API + '/commands');

        return res.json();
    }

    async upsertCommand(command) {
        const res = await fetch(process.env.REACT_APP_API + '/command', {method: 'POST', body: JSON.stringify(command)});

        return res.ok;
    }

    async deleteCommand(name) {
        const res = await fetch(process.env.REACT_APP_API + `/command/${name}`, {method: 'DELETE'});

        return res.ok;
    }
}

export default new Api();