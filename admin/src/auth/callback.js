import { useEffect, useState } from "react";
import { Navigate, useSearchParams } from "react-router-dom";
import Api from "../api/Api";

export default function Callback(props) {
    const [redirect, setRedirect] = useState(false);

    const searchParams = useSearchParams();

    useEffect(() => {
        Api.get_token(searchParams.toString()).then(x => {
            let expiration_time = new Date(Date.now());
            expiration_time.setSeconds(expiration_time.getSeconds() + x.expires_in);

            localStorage.setItem("token", JSON.stringify({ token: x.token, expiration_time: expiration_time.getTime() }));

            setRedirect(true);
        });
    }, []);

    if (redirect) {
        return <Navigate to="/admin/" />;
    }

    return "Loading...";
}