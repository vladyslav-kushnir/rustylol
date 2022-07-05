import { useEffect, useState } from "react";
import Api from "./api/Api";
import Variation from "./Variation";

export default function Command(props) {
    const [variations, setVariations] = useState(Object.values(props.variations).map(x => { return { initial: x.url_pattern, current: x.url_pattern, isValid: !!x.url_pattern }; }));
    const [name, setName] = useState(props.name);
    const [canSave, setCanSave] = useState(false);
    const [canDeleteVariations, setCanDeleteVariations] = useState(variations.filter(x => !x.isDeleted).length > 1);
    const [isNew, setIsNew] = useState(props.isNew || false);

    let onVariantChange = (idx, value) => {
        const newVariations = [...variations];
        newVariations[idx] = { ...variations[idx], current: value, isValid: !!value};

        setVariations(newVariations);
    };

    let onVariantDelete = (idx) => {
        if (!canDeleteVariations) {
            return;
        }

        const newVariations = [...variations];
        if (variations[idx].isNew) {
            // Pre-existing
            newVariations[idx] = { ...variations[idx], current: '', isDeleted: true, isValid: true };
        }
        else {
            // Newly created
            newVariations.splice(idx, 1);
        }

        setVariations(newVariations);
    };

    let addVariation = () => {
        setVariations([...variations, { initial: '', current: '', isNew: true, isValid: false }]);
    };

    useEffect(() => {
        setCanDeleteVariations(variations.filter(x => !x.isDeleted).length > 1);

        const nameIsValid = !!name;
        const variationsAreValid = variations.every(x => x.isValid);

        // Exit early
        if (!nameIsValid || !variationsAreValid) {
            setCanSave(false);
            return;
        }

        const nameChanged = props.name != name;
        const variationsChanged = variations.some(x => x.isDeleted || x.initial != x.current);

        setCanSave(nameChanged || variationsChanged);
    }, [variations, name]);

    let save = async () => {
        if (await Api.upsertCommand({
            name,
            variations: variations.filter(x => !x.isDeleted).map(x => x.current)
        })) {
            setIsNew(false);
            setCanSave(false);
        }
    };

    let deleteCommand = async () => {
        if (await Api.deleteCommand(name)) {
            props.onDelete();
        }
    };

    return [
        <div key={'data'} style={{ display: 'flex', padding: 5, gap: 10 }}>
            <div>Name: <input value={name} onChange={change => setName(change.target.value)} className={!name ? 'invalid' : undefined}/></div>
            <div>Variations:</div>
            <div>
                {variations.map((x, idx) => !x.isDeleted ? <Variation key={idx} urlPattern={x.current} onChange={onVariantChange.bind(this, idx)} onDelete={onVariantDelete.bind(this, idx)} canDelete={canDeleteVariations} isValid={x.isValid}/> : undefined).filter(x => x)}
                <div><button onClick={addVariation}>Add variation</button></div>
            </div>
            <div style={{marginLeft: 10, alignSelf: 'center', display: 'flex', gap: 10 }}>
                <button disabled={!canSave} onClick={save}>Save</button>
                {!isNew &&<button onClick={deleteCommand}>Delete</button> }
            </div>
        </div>,
        <hr key={'line'} />
    ];
}