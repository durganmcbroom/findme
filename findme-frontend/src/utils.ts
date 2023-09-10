import {DisasterType} from "./fetch.ts";

export function stringDateToMilis(dateString: string) : number {
    const [mm, dd, yyyy] = dateString.split("-").map(Number);

    const date = new Date(yyyy, mm - 1, dd);

    return date.getTime()
}

export function stringDisasterToEnum(disaster: string) : DisasterType {
    switch (disaster) {
        case "CarCrash":
            return DisasterType.CarCrash
        default:
            return DisasterType.CarCrash
    }
}

export  function prettyMilis(milis: number): string {
    return new Date(milis).toLocaleString(undefined, {
        year: 'numeric',
        month: 'long',
        day: 'numeric'
    })
}