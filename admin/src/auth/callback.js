import { useEffect, useState } from "react";
import { Navigate, useSearchParams } from "react-router-dom";
import Api from "../api/Api";

export default function Callback(props) {
    const [redirect, setRedirect] = useState(false);

    const searchParams = useSearchParams();

    useEffect(() => {
        Api.get_token(searchParams.toString()).then(x => {

            localStorage.setItem("token", x);

            setRedirect(true);
        });
    }, []);

    if (redirect) {
        return <Navigate to="/" />;
    }

    return "Loading...";
}