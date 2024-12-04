shader_type sky;

#define iterations 15
#define formuparam 0.45

#define volsteps 10
#define stepsize 0.3

#define zoom  0.0001
#define tile   0.000
#define speed  0.010

#define brightness 0.0003
#define darkmatter 0.300
#define distfading 0.530
#define saturation 0.650


void sky() {
    // vec3 direction = EYEDIR * vec3(SKY_COORDS,SKY_COORDS.x);
    // vec3 direction = vec3(SCREEN_UV, 0) + EYEDIR;
    vec3 direction = EYEDIR;
    vec3 position = POSITION*zoom;
    float s  = 0.4;
    float fade = 1.0;
    vec3 v=vec3(0.0);

    for (int r=0; r<volsteps; r++) {
        vec3 p=position+s*direction*.5;
        p = abs(vec3(tile)-mod(p,vec3(tile*2.))); // tiling fold
        float pa,a=pa=0.;
        for (int i=0; i<iterations; i++) {
            p=abs(p)/dot(p,p)-formuparam; // the magic formula
            a+=abs(length(p)-pa); // absolute sum of average change
            pa=length(p);
        }
        float dm=max(0.,darkmatter-a*a*.001); //dark matter
        a*=a*a; // add contrast
        if (r>6) fade*=1.-dm; // dark matter, don't render near
        //v+=vec3(dm,dm*.5,0.);
        v+=fade;
        v+=vec3(s,s*s,s*s*s*s)*a*brightness*fade; // coloring based on distance
        fade*=distfading; // distance fading
        s+=stepsize;
    }
    v=mix(vec3(length(v)),v,saturation); //color adjust
    COLOR = vec3(v*.01);
}



//void sky()
//{
    ////get coords and direction
    //vec3 iResolution = POSITION / EYEDIR;
    //vec2 uv=FRAGCOORD.xy/iResolution.xy-.5;
    //uv.y*=iResolution.y/iResolution.x;
    //vec3 dir=vec3(uv*zoom,1.);
    //// float time=iTime*speed+.25;
    //vec3 from=vec3(1.,.5,0.5);
    ////volumetric rendering
    //float s=0.1,fade=1.;
    //vec3 v=vec3(0.);
    //for (int r=0; r<volsteps; r++) {
        //vec3 p=from+s*dir*.5;
        //p = abs(vec3(tile)-mod(p,vec3(tile*2.))); // tiling fold
        //float pa,a=pa=0.;
        //for (int i=0; i<iterations; i++) {
            //p=abs(p)/dot(p,p)-formuparam; // the magic formula
            //a+=abs(length(p)-pa); // absolute sum of average change
            //pa=length(p);
        //}
        //float dm=max(0.,darkmatter-a*a*.001); //dark matter
        //a*=a*a; // add contrast
        //if (r>6) fade*=1.-dm; // dark matter, don't render near
        ////v+=vec3(dm,dm*.5,0.);
        //v+=fade;
        //v+=vec3(s,s*s,s*s*s*s)*a*brightness*fade; // coloring based on distance
        //fade*=distfading; // distance fading
        //s+=stepsize;
    //}
    //v=mix(vec3(length(v)),v,saturation); //color adjust
    //COLOR = vec3(v*.01);
//}
