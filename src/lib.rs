mod vector2;

use std::cmp::Ordering;
use std::f64::consts::{PI, TAU};
pub use vector2::Vector2;

#[derive(Clone, Debug)]
pub struct World {
    pub vertexes: Vec<Vector2>,
}

impl World {
    #[inline]
    pub fn new(vertexes: Vec<Vector2>) -> World {
        World { vertexes }
    }
}

pub struct Camera {
    pub position: Vector2,
    // 弧度制
    pub rotation: f64,
    pub y_fixpoint: f64,
    pub z_fixpoint: f64,
    pub vertex_height: f64,
    pub translation_speed: f64,
    pub rotation_speed: f64,
}

impl Camera {
    fn to_orthographic(&self, world: World) -> World {
        World::new(
            world
                .vertexes
                .into_iter()
                .map(|Vector2(x, y)| Vector2(x * self.y_fixpoint / y, y))
                .collect(),
        )
    }

    /// 生成一个相机视角下的世界，使得所有顶点以正交视角呈现，并切除负y值的顶点。
    /// 返回的世界中的vertexes中的向量的第一分量(v.1)转变为高度值，即2z。
    pub fn see(&self, world: World) -> World {
        let mut result = World::new(
            world
                .vertexes
                .into_iter()
                .map(|v| v - self.position) // 平移所有顶点
                .map(|v| v.rotate(-self.rotation)) // 绕相机旋转所有顶点至正方向（Vector2::forward()）
                .collect(),
        );
        // 切除负y值的顶点
        {
            use Ordering::*;
            assert!(result.vertexes.len() >= 3);
            {
                let (&u, &v) = (
                    result.vertexes.first().unwrap(),
                    result.vertexes.last().unwrap(),
                );
                match v.1.partial_cmp(&0.).unwrap() {
                    Less => {
                        if u.1 > 0. {
                            let v = result.vertexes.last_mut().unwrap();
                            v.0 = v.0 - (u.0 - v.1) * u.1 / (u.1 - v.1); // 取两点之间连线与x轴的交点
                            v.1 = 0.;
                        }
                    }
                    Greater => {
                        if u.1 < 0. {
                            let u = result.vertexes.first_mut().unwrap();
                            u.0 = v.0 - (u.0 - v.1) * u.1 / (u.1 - v.1); // 取两点之间连线与x轴的交点
                            u.0 = 0.;
                        }
                    }
                    Equal => {}
                }
            }
            for i in 0..result.vertexes.len() - 1 {
                let v = result.vertexes[i];
                let u = result.vertexes[i + 1];
                match v.1.partial_cmp(&0.).unwrap() {
                    Equal => {}
                    Less => match u.1.partial_cmp(&0.).unwrap() {
                        Less | Equal => {
                            let v = &mut result.vertexes[i];
                            v.1 = 0.;
                        }
                        Greater => {
                            let v = &mut result.vertexes[i];
                            v.0 = v.0 - (u.0 - v.1) * u.1 / (u.1 - v.1); // 取两点之间连线与x轴的交点
                            v.1 = 0.;
                        }
                    },
                    Greater => {
                        if u.1 < 0. {
                            let u = &mut result.vertexes[i + 1];
                            u.0 = v.0 - (u.0 - v.1) * u.1 / (u.1 - v.1); // 取两点之间连线与x轴的交点
                            u.1 = 0.;
                        }
                    }
                }
            }
        }
        let result = self.to_orthographic(result);
        let result = World::new(
            result
                .vertexes
                .into_iter()
                .filter(|&Vector2(x, y)| !x.is_nan())
                .collect(),
        );
        // 将y值转为高度值
        World::new(
            result
                .vertexes
                .into_iter()
                .map(|v| Vector2(v.0, self.z_fixpoint * self.vertex_height / v.1))
                .collect(),
        )
    }

    #[inline]
    pub fn rotate(&mut self, radian: f64) {
        self.rotation += radian * self.rotation_speed;
        if self.rotation >= PI {
            self.rotation -= TAU;
        } else if self.rotation <= -PI {
            self.rotation += TAU;
        }
    }

    #[inline]
    pub fn translate(&mut self, v: Vector2) {
        self.position += v * self.translation_speed;
    }
}
