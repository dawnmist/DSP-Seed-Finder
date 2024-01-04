import { GasType, RuleType, VeinType } from "./enums"

export function toPrecision(number: number, precision: number) {
    return number.toLocaleString([], {
        minimumFractionDigits: 0,
        maximumFractionDigits: precision,
    })
}

export function formatNumber(number: number, precision: number): string {
    if (number >= 1e6) {
        return toPrecision(number / 1e6, precision) + "M"
    } else if (number >= 1e4) {
        return toPrecision(number / 1e3, precision) + "K"
    } else {
        return toPrecision(number, precision)
    }
}

function fixRule(rule: SimpleRule): SimpleRule {
    if (
        rule.type === RuleType.AverageVeinAmount &&
        rule.vein === VeinType.Oil
    ) {
        return {
            ...rule,
            condition: {
                ...rule.condition,
                value: rule.condition.value * 25e3,
            },
        }
    }
    return rule
}

export function constructRule(rules: SimpleRule[][]): Rule {
    const rs: Rule[] = rules.map((r) =>
        r.length === 1
            ? fixRule(r[0]!)
            : { type: RuleType.Or, rules: r.map(fixRule) },
    )
    return rs.length === 1 ? rs[0]! : { type: RuleType.And, rules: rs }
}

export const minStarCount = 32
export const maxStarCount = 64
export const defaultStarCount = 64

export const resourceMultiplers: ReadonlyArray<float> = [
    0.1, 0.5, 0.8, 1, 1.5, 2, 3, 5, 8, 100,
]
export const defaultResourceMultipler = 1

export const veinNames: Record<VeinType, string> = {
    [VeinType.None]: "",
    [VeinType.Iron]: "Iron Ore",
    [VeinType.Copper]: "Copper Ore",
    [VeinType.Silicium]: "Silicon Ore",
    [VeinType.Titanium]: "Titanium Ore",
    [VeinType.Stone]: "Stone",
    [VeinType.Coal]: "Coal",
    [VeinType.Oil]: "Crude Oil",
    [VeinType.Fireice]: "Fire Ice",
    [VeinType.Diamond]: "Kimberlite Ore",
    [VeinType.Fractal]: "Fractal Silicon",
    [VeinType.Crysrub]: "Organic Crystal",
    [VeinType.Grat]: "Grating Crystal",
    [VeinType.Bamboo]: "Stalagmite Crystal",
    [VeinType.Mag]: "Unipolar Magnet",
}

export const gasNames: Record<GasType, string> = {
    [GasType.None]: "",
    [GasType.Fireice]: "Fire Ice",
    [GasType.Hydrogen]: "Hydrogen",
    [GasType.Deuterium]: "Deuterium",
}

export function getSearch({
    count,
    multipler,
}: {
    count: integer
    multipler: float
}) {
    const params = new URLSearchParams()
    if (count !== defaultStarCount) {
        params.set("count", String(count))
    }
    if (multipler !== defaultResourceMultipler) {
        params.set("multipler", String(multipler))
    }
    const str = params.toString()
    return str ? "?" + str : ""
}

export const planetTypes: Record<number, string> = {
    1: "Mariterra",
    6: "Scorchedia",
    7: "Geloterra",
    8: "Tropicana",
    9: "Lava",
    10: "Glacieon",
    11: "Desolus",
    12: "Gobi",
    13: "Sulfuria",
    14: "Crimsonis",
    15: "Prairiea",
    16: "Aquatica",
    17: "Halitum",
    18: "Sakura Ocean",
    19: "Cyclonius",
    20: "Maroonfrost",
    22: "Savanna",
    23: "Onyxtopia",
    24: "Icefrostia",
    25: "Pandora Swamp",
}
