/* A %KinematicsVector is a container class used to report kinematics data for
registered frames and geometries (keyed by unique FrameId/GeometryId values)
to SceneGraph where the set of keys (FrameId/GeometryId) is usually constant
and the values (kinematics data) are varying. It is an internal class and one
should never interact with it directly. The template aliases FramePoseVector
and GeometryConfigurationVector that instantiate %KinematicsVector should be
used instead.

<!--
  TODO(SeanCurtis-TRI): The FrameVelocityVector and FrameAccelerationVector
  are still to come.
 -->

```
template <typename T>
class MySystem : public LeafSystem<T> {
 public:
  MySystem() {
    ...
    this->DeclareAbstractOutputPort(
        &AllocInConstructorSystem::CalcFramePoseOutput);
    ...
  }

 private:
  void CalcFramePoseOutput(const Context<T>& context,
                           geometry::FramePoseVector<T>* poses) const {
    poses->clear();
    for (int i = 0; i < static_cast<int>(frame_ids_.size()); ++i) {
      poses->set_value(frame_ids_[i], poses_[i]);
    }
  }

  std::vector<FrameId> frame_ids_;
  std::vector<RigidTransform<T>> poses_;
};
```

If a System only ever emits a single frame/geometry (or small-constant-number
of frames/geometries), then there's a shorter alternative way to write a Calc
method, using an initializer_list:
```
  void CalcFramePoseOutput(const Context<T>& context,
                           geometry::FramePoseVector<T>* poses) const {
    const RigidTransform<T>& pose = ...;
    *poses = {{frame_id_, pose}};
  }
```

N.B. When the systems framework calls the `Calc` method, the value pointed to
by `poses` is in an unspecified state.  The implementation of `Calc` must
always ensure that `poses` contains the correct value upon return, no matter
what value it started with.  The easy ways to do this are to call either
`poses->clear()` or the assignment operator `*poses = ...`.

@tparam Id               The key used to locate the kinematics data. Can be
                         FrameId or GeometryId.
@tparam KinematicsValue  The underlying data type of the kinematics data (e.g.,
                         pose, configuration, or velocity).

The FramePoseVector and GeometryConfigurationVector classes are aliases of the
%KinematicsVector instantiated on specific data types (RigidTransform and
VectorX respectively). Each of these data types are templated on Eigen scalars.
All supported combinations of data type and scalar type are already available
to link against in the containing library. No other values for KinematicsValue
are supported.

Currently, the following data types with the following scalar types are
supported:

 Alias                               | Instantiation                                    | Scalar types
-------------------------------------|--------------------------------------------------|-------------
 FramePoseVector<Scalar>             | KinematicsVector<FrameId,RigidTransform<Scalar>> | double/AutoDiffXd/Expression
 GeometryConfigurationVector<Scalar> | KinematicsVector<GeometryId, VectorX<Scalar>>    | double/AutoDiffXd/Expression
*/
