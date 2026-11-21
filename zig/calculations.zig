const std = @import("std");
const bhaskara_result = extern struct {
    x1: f64,
    x2: f64,
};
const vertical_result = extern struct {
    x: f64,
    y: f64,
};
const divide_result = extern struct {
    ab: f64,
};
export fn bhaskara(a: f64,b: f64,c: f64,result: *bhaskara_result) callconv(.c) bool {
    const delta = b*b-4*a*c;
    if (a == 0) {return false;}
    else if (delta < 0) {
        return false;
    }
    else {
        const delta_root = @sqrt(delta);
        const x1 = (-b+delta_root)/(2*a);
        const x2 = (-b-delta_root)/(2*a);
        result.* = .{
            .x1 = x1,
            .x2 = x2,
        };
        return true;
    }
}
export fn sum(a: f64,b: f64) callconv(.c) f64 {
    return a+b;
}
export fn subtract(a: f64,b: f64) callconv(.c) f64 {
    return a-b;
}
export fn multiply(a: f64,b: f64) callconv(.c) f64 {
    return a*b;
}
export fn divide(a: f64,b: f64,result: *divide_result) callconv(.c) bool {
    if (b == 0) {
        return false;
    }
    else {
        const ab = a/b;
        result.* = .{
            .ab = ab,
        };
        return true;
    }
}
export fn volume_cylinder(pi: f64,r: f64,h: f64) callconv(.c) f64 {
    const volume = pi*r*r*h;
    return volume;
}
export fn vertical(a: f64,b: f64,c: f64,result: *vertical_result) callconv(.c) bool {
    if (a == 0) {
        return false;
    }
    else {
        const x = -b/(2*a);
        const delta_for_divide = b*b-4*a*c;
        const y = -delta_for_divide/(4*a);
        result.* = .{
            .x = x,
            .y = y,
        };
        return true;
    }
}
export fn pitagoras(hipotenuse: f64,cateto_oposto: f64,cateto_adjacente: f64) callconv(.c) f64 {
    if (hipotenuse == 1 and cateto_oposto > 1 and cateto_adjacente > 1) {
        const sum_pitagoras = (cateto_oposto*cateto_oposto)+(cateto_adjacente*cateto_adjacente);
        const final_result = @sqrt(sum_pitagoras);
        return final_result;
    }
    else if (hipotenuse > 1 and cateto_oposto == 1 and cateto_adjacente > 1) {
        const subtract_pitagoras = (hipotenuse*hipotenuse)+(cateto_oposto*cateto_oposto);
        const final_subtract = @sqrt(subtract_pitagoras);
        return final_subtract;
    }
    else if (hipotenuse > 1 and cateto_oposto > 1 and cateto_adjacente == 1) {
        const subtract2 = (hipotenuse*hipotenuse)+(cateto_oposto*cateto_oposto);
        const final_subtract2 = @sqrt(subtract2);
        return final_subtract2;
    }
    else {
        const all_sum = (hipotenuse*hipotenuse)+(cateto_adjacente*cateto_adjacente)+(cateto_oposto*cateto_oposto);
        const final_all_sum = @sqrt(all_sum);
        return final_all_sum;
    }
}
