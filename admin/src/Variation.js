export default function Variation(props) {
    return <div>
        <input value={props.urlPattern} onChange={change => props.onChange(change.target.value)} />
        {props.canDelete && <button onClick={props.onDelete}>X</button>}
    </div>;
}