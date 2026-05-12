use crate::{cube::UnitCube, math::{Mat4, Vec3}};
use crate::stack::MatrixStack;

#[derive(Clone, Copy, Debug)]
pub enum Motion {
    Idle,
    Walk,
    Jump,
}

#[derive(Clone, Copy, Debug)]
pub struct Pose {
    pub root_y: f32,
    pub hip_l: f32, pub hip_r: f32,
    pub knee_l: f32, pub knee_r: f32,
    pub shoulder_l: f32, pub shoulder_r: f32,
    pub elbow_l: f32, pub elbow_r: f32,
}

pub struct HumanAnimation {
    pub motion: Motion,
    pub walk_phase: f32,
    pub jump_t: f32,
}

impl HumanAnimation {
    pub fn new() -> Self {
       Self {motion: Motion::Idle, walk_phase: 0.0, jump_t: 0.0}
    }
    pub fn start_jump(&mut self) {
        self.motion = Motion::Jump;
        self.jump_t = 0.0;
    }

    pub fn update(&mut self, dt: f32) -> Pose {
        // ----------------------------
        // Step 4: update timers
        // ----------------------------
        let walk_speed: f32 = 2.5;     // rad/sec
        let jump_duration: f32 = 0.7;  // seconds
    
        match self.motion {
            Motion::Idle => {}
            Motion::Walk => {
                self.walk_phase += dt * walk_speed;
    
                let tau = std::f32::consts::TAU;
                if self.walk_phase > tau {
                    self.walk_phase -= tau;
                }
            }
            Motion::Jump => {
                self.jump_t += dt / jump_duration;
    
                if self.jump_t >= 1.0 {
                    self.jump_t = 0.0;
                    self.motion = Motion::Idle;
                }
            }
        }
    
        // ----------------------------
        // Step 5: compute pose
        // ----------------------------
        let s = self.walk_phase.sin();
        let c = self.walk_phase.cos();
    
        let hip_amp = 0.5;
        let shoulder_amp = 0.6;
        let knee_max = 1.2;
        let elbow_bend = 0.4;
    
        let jump_height = 1.0;
    
        let mut pose = Pose {
            root_y: 0.0,
    
            hip_l: 0.0,
            hip_r: 0.0,
            knee_l: 0.0,
            knee_r: 0.0,
    
            shoulder_l: 0.0,
            shoulder_r: 0.0,
            elbow_l: elbow_bend,
            elbow_r: elbow_bend,
        };
    
        match self.motion {
            Motion::Idle => {
                // optional subtle sway (comment out if you want totally still)
                // pose.shoulder_l = 0.1 * (self.walk_phase * 0.5).sin();
                // pose.shoulder_r = -pose.shoulder_l;
            }
    
            Motion::Walk => {
                // hips swing opposite each other
                pose.hip_l = -hip_amp * s;
                pose.hip_r = hip_amp * s;
    
                // knees bend in the middle of the stride (cos is phase-shifted vs sin)
                pose.knee_l = knee_max * (0.0f32).max(c);
                pose.knee_r = knee_max * (0.0f32).max(-c);
    
                // arms counter-swing
                pose.shoulder_l = -shoulder_amp * s;
                pose.shoulder_r = shoulder_amp * s;
    
                // keep elbows slightly bent
                pose.elbow_l = elbow_bend;
                pose.elbow_r = elbow_bend;
            }
    
            Motion::Jump => {
                let p = self.jump_t.clamp(0.0, 1.0);
            
                // vertical motion
                pose.root_y = jump_height * (4.0 * p * (1.0 - p)); // 0 -> peak -> 0
            
                // knees: bend most at the top (p=0.5), straight at start/end
                // this is a triangle-shaped curve peaking at 0.5
                let topness = 1.0 - (2.0 * p - 1.0).abs(); // 0 at ends, 1 at top
                let knee_bend = 0.4 * topness;             // tweak 0.4 if you want more/less
            
                pose.knee_l = knee_bend;
                pose.knee_r = knee_bend;
            
                // hips mostly neutral (optional tiny bend)
                pose.hip_l = -0.1 * topness;
                pose.hip_r = -0.1 * topness;
            
                // arms can stay neutral or slightly lift
                // pose.shoulder_l = 0.2 * topness;
                // pose.shoulder_r = 0.2 * topness;
            }
        }
    
        // ----------------------------
        // clamps (human-ish)
        // ----------------------------
        pose.hip_l = pose.hip_l.clamp(-1.0, 1.0);
        pose.hip_r = pose.hip_r.clamp(-1.0, 1.0);
    
        // knees should not go negative in this rig
        pose.knee_l = pose.knee_l.clamp(0.0, 2.2);
        pose.knee_r = pose.knee_r.clamp(0.0, 2.2);
    
        pose.shoulder_l = pose.shoulder_l.clamp(-1.2, 1.2);
        pose.shoulder_r = pose.shoulder_r.clamp(-1.2, 1.2);
    
        pose.elbow_l = pose.elbow_l.clamp(0.0, 2.2);
        pose.elbow_r = pose.elbow_r.clamp(0.0, 2.2);
    
        pose
    }
}

pub fn draw_human(
    cube: &UnitCube,
    proj: Mat4,
    view: Mat4,
    debug_spin: bool,
    t_now: f32,
    pose: Pose,
) {
    let mut ms = MatrixStack::new();

    ms.apply(Mat4::translation(0.0, pose.root_y, 0.0));
    if debug_spin {
        ms.apply(Mat4::rotation_y(t_now * 0.5));
    }
    let torso_w = 1.0;
    let torso_h = 1.5;
    let torso_d = 0.5;

    ms.push();
    ms.apply(Mat4::scale(torso_w, torso_h, torso_d));
    let mvp = Mat4::mul(proj, Mat4::mul(view, ms.top()));
    cube.draw(&mvp, [0.2, 0.7, 0.35]);
    ms.pop();

    let head_h = 0.6;
    
    ms.push();
    ms.apply(Mat4::translation(0.0, torso_h * 0.5, 0.0));
    ms.apply(Mat4::translation(0.0, head_h * 0.5, 0.0));
    ms.apply(Mat4::scale(0.6, head_h, 0.6));
    let mvp = Mat4::mul(proj, Mat4::mul(view, ms.top()));
    cube.draw(&mvp, [0.9, 0.78, 0.65]);
    ms.pop();

    let upper_arm_h = 0.8;
    let upper_arm_w = 0.25;

    let forearm_h = 0.7;
    let forearm_w = 0.22;

    let shoulder_x = torso_w * 0.5 + upper_arm_w * 0.5;
    let shoulder_y = torso_h * 0.5 - 0.05;

    ms.push();
//move to shoulder position
    ms.apply(Mat4::translation(shoulder_x, shoulder_y, 0.0));
    //rotate upper arm at shoulder
    ms.apply(Mat4::rotation_x(pose.shoulder_r));

    ms.push();
    ms.apply(Mat4::translation(0.0, -upper_arm_h * 0.5, 0.0));
    ms.apply(Mat4::scale(upper_arm_w, upper_arm_h, upper_arm_w));
    let mvp = Mat4::mul(proj, Mat4::mul(view, ms.top()));
    cube.draw(&mvp, [0.2, 0.7, 0.35]);
    ms.pop();

    //move to elbow
    ms.apply(Mat4::translation(0.0, -upper_arm_h, 0.0));

    //rotate forearm at elbow
    ms.apply(Mat4::rotation_x(pose.elbow_r));

    ms.push();
    ms.apply(Mat4::translation(0.0, -forearm_h * 0.5, 0.0));
    ms.apply(Mat4::scale(forearm_w, forearm_h, forearm_w));
    let mvp = Mat4::mul(proj, Mat4::mul(view, ms.top()));
    cube.draw(&mvp, [0.90, 0.78, 0.65]);
    ms.pop();

    ms.pop();

    ms.push();
    //move to leftshoulder position
        ms.apply(Mat4::translation(-shoulder_x, shoulder_y, 0.0));
        //rotate upper arm at shoulder
        ms.apply(Mat4::rotation_x(pose.shoulder_l));
    
        ms.push();
        ms.apply(Mat4::translation(0.0, -upper_arm_h * 0.5, 0.0));
        ms.apply(Mat4::scale(upper_arm_w, upper_arm_h, upper_arm_w));
        let mvp = Mat4::mul(proj, Mat4::mul(view, ms.top()));
        cube.draw(&mvp, [0.2, 0.7, 0.35]);
        ms.pop();
    
        //move to elbow
        ms.apply(Mat4::translation(0.0, -upper_arm_h, 0.0));
    
        //rotate forearm at elbow
        ms.apply(Mat4::rotation_x(pose.elbow_l));
    
        ms.push();
        ms.apply(Mat4::translation(0.0, -forearm_h * 0.5, 0.0));
        ms.apply(Mat4::scale(forearm_w, forearm_h, forearm_w));
        let mvp = Mat4::mul(proj, Mat4::mul(view, ms.top()));
        cube.draw(&mvp, [0.9, 0.78, 0.65]);
        ms.pop();
    
        ms.pop();

        //lower body
        let thigh_h = 0.9;
        let thigh_w = 0.3;

        let shin_h = 0.8;
        let shin_w = 0.26;

        let hip_x = torso_w * 0.25;
        let hip_y = -torso_h * 0.5;

        ms.push();

        ms.apply(Mat4::translation(hip_x, hip_y, 0.0));
        ms.apply(Mat4::rotation_y(std::f32::consts::PI)); // flip leg facing
        ms.apply(Mat4::rotation_x(pose.hip_r));
        
        ms.push();
        ms.apply(Mat4::translation(0.0, -thigh_h * 0.5, 0.0));
        ms.apply(Mat4::scale(thigh_w, thigh_h, thigh_w));
        let mvp = Mat4::mul(proj, Mat4::mul(view, ms.top()));
        cube.draw(&mvp, [0.18, 0.35, 0.70]);
        ms.pop();

        //move to knee
        ms.apply(Mat4::translation(0.0, -thigh_h, 0.0));
        ms.apply(Mat4::rotation_x(pose.knee_r));

        ms.push();
        ms.apply(Mat4::translation(0.0, -shin_h * 0.5, 0.0));
        ms.apply(Mat4::scale(shin_w, shin_h, shin_w));
        let mvp = Mat4::mul(proj, Mat4::mul(view, ms.top()));
        cube.draw(&mvp, [0.18, 0.35, 0.70]);
        ms.pop();

        ms.pop();

        ms.push();

        ms.apply(Mat4::translation(-hip_x, hip_y, 0.0));
        ms.apply(Mat4::rotation_y(std::f32::consts::PI)); // flip leg facing
        ms.apply(Mat4::rotation_x(pose.hip_l));
        
        ms.push();
        ms.apply(Mat4::translation(0.0, -thigh_h * 0.5, 0.0));
        ms.apply(Mat4::scale(thigh_w, thigh_h, thigh_w));
        let mvp: Mat4 = Mat4::mul(proj, Mat4::mul(view, ms.top()));
        cube.draw(&mvp, [0.18, 0.35, 0.70]);
        ms.pop();

        //move to knee
        ms.apply(Mat4::translation(0.0, -thigh_h, 0.0));
        ms.apply(Mat4::rotation_x(pose.knee_l));

        ms.push();
        ms.apply(Mat4::translation(0.0, -shin_h * 0.5, 0.0));
        ms.apply(Mat4::scale(shin_w, shin_h, shin_w));
        let mvp = Mat4::mul(proj, Mat4::mul(view, ms.top()));
        cube.draw(&mvp, [0.18, 0.35, 0.70]);
        ms.pop();

        ms.pop();
}