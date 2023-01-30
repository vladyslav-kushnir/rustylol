class AuthError extends Error {}

class Api {
    getAuthToken() {
        const token = localStorage.getItem('token');
        if (!token) {
            localStorage.removeItem('token');

            throw new AuthError();
        }

        const parsed_token = JSON.parse(token);

        if (parsed_token.expiration_time <= Date.now()) {
            localStorage.removeItem('token');

            throw new AuthError();
        }

        return parsed_token.token;
    }

    async getCommands() {
        const res = await fetch(process.env.REACT_APP_API + '/commands', { 
            headers: {
                'Authorization': `Bearer ${this.getAuthToken()}`
            } 
        });

        if (res.status == 401) {
            localStorage.removeItem('token');

            throw new AuthError();
        }

        return res.json();
    }

    async upsertCommand(command) {
        const res = await fetch(process.env.REACT_APP_API + '/command', {
            method: 'POST', body: JSON.stringify(command),
            headers: {
                'Authorization': `Bearer ${this.getAuthToken()}`
            } 
        });

        if (res.status == 401) {
            localStorage.removeItem('token');

            throw new AuthError();
        }

        return res.ok;
    }

    async deleteCommand(name) {
        const res = await fetch(process.env.REACT_APP_API + `/command/${name}`, {
            method: 'DELETE',
            headers: {
                'Authorization': `Bearer ${this.getAuthToken()}`
            } 
        });

        if (res.status == 401) {
            localStorage.removeItem('token');

            throw new AuthError();
        }

        return res.ok;
    }

    async get_auth_url() {
        const res = await fetch(process.env.REACT_APP_API + `/auth/azure/url`);

        return res.text();
    }

    async get_token(args) {
        const res = await fetch(process.env.REACT_APP_API + `/auth_callback?${args}`);

        return res.json();
    }
}

export default new Api();