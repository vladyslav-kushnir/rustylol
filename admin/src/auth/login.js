import { useEffect, useState } from "react";
import Api from "../api/Api";

export default function Login() {
    const [redirect, setRedirect] = useState('');

    let onLogin = async () => {
        const redirectUrl = await Api.get_auth_url();

        setRedirect(redirectUrl);
    }

    if (redirect) {
        window.location.href = redirect;
    }

    return <button onClick={onLogin}>Login</button>;
}